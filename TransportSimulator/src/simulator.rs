use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use crate::roadmap::{RegulatedRoad, ShortestPath, Distance};
use crate::vehicle::Intention;

trait TrafficLight {
    fn permit(&self, from: &RegulatedRoad, to: &RegulatedRoad, tso: i64) -> bool;
}

struct RoundRobinTrafficLight {
    pub interval: i64,
}

impl TrafficLight for RoundRobinTrafficLight {
    fn permit(&self, from: &RegulatedRoad, to: &RegulatedRoad, tso: i64) -> bool {
        let turn = (tso / self.interval) % (to.inbounds.len() as i64);
        true
    }
}

struct BlockContext {
    vehicle: Box<dyn vehicle::Vehicle>,
    arrived_tso: i32,
}

// Traffic on one road
struct RoadTraffic {
    road: & 'static RegulatedRoad,

    // pending_vehicles shall be empty after every tick.
    // vehicles is either running or blocked on traffic light.
    pending_vehicles: Vec<Box<dyn vehicle::Vehicle>>,
    running_vehicles: Vec<Box<dyn vehicle::Vehicle>>,
    blocked_vehicles: Vec<BlockContext>,
    tombstone_vehicles: Vec<Box<dyn vehicle::Vehicle>>,

    traffic_light: dyn TrafficLight,

}

#[derive(Default)]
struct RoadTrafficResult {
    pub pending_vehicles: Vec<(i32, Box<dyn vehicle::Vehicle>)>,
}

impl RoadTraffic {
    fn handle1(&mut self, tso: i64) -> RoadTrafficResult {
        // At the beginning of a tick, there are no elements in `pending_vehicles`.
        // However, there will be lots of tick in one
        assert_eq!(self.running_vehicles.len(), 0);
        // Vehicles that we need to send to other roads.
        let mut res: RoadTrafficResult = Default::default();
        enum Dest {
            Running,
            Pending(i32),
            TombStone,
        };
        // For every vehicle in `running_vehicles`, we want to see if it can be send to other roads.
        let running_vehicles = std::mem::take(&mut self.running_vehicles);
        for mut v in running_vehicles.into_iter() {
            let ability = v.get_left_equivalent_distance();
            let v_id = v.get_id();
            let intention = v.intention_mut();
            let dest = match intention {
                vehicle::Intention::Die => {
                    Dest::TombStone
                },
                vehicle::Intention::Goto(ref mut g) => {
                    assert_eq!(g.from.road, self.road.id);
                    if ability >= self.road.weight - g.from.offset {
                        // We can go to next road
                        let next = if g.via.is_empty() {
                            if g.from.road == g.to.road {
                                // Shall die
                                None
                            }else{
                                g.from.road = g.to.road;
                                g.from.offset = 0;
                                Some(g.to.road)
                            }
                        } else {
                            let r = g.via.pop().unwrap();
                            g.from.road = r;
                            g.from.offset = 0;
                            Some(r)
                        };
                        if next.is_none() {
                            Dest::TombStone
                        } else {
                            Dest::Pending(next.unwrap())
                        }
                    } else {
                        g.from.offset += ability.clone();
                        v.go_distance(ability);
                        Dest::Running
                    }
                },
            };
            match dest {
                Dest::Running => {
                    println!("[tso {}] Vehicle {} stops at road {}", tso, v_id, self.road.id);
                    self.running_vehicles.push(v);
                    ()
                },
                Dest::TombStone => {
                    println!("[tso {}] Vehicle {} die", tso, v_id);
                    self.tombstone_vehicles.push(v);
                    ()
                },
                Dest::Pending(n) => {
                    println!("[tso {}] Vehicle {} pending resolution from road {}", tso, v_id, self.road.id);
                    res.pending_vehicles.push((n, v));
                    ()
                }
            }
        }

        res
    }
}

#[derive(Default)]
struct Traffic {
    roads: HashMap<i32, Box<RoadTraffic>>, // road_id -> RoadTraffic
}

struct Simulator {
    regulated_roadmap: roadmap::RegulatedRoadMap,
    shortest_path: Option<HashMap<(i32, i32), roadmap::ShortestPath>>,
    traffic: Traffic,
    // All vehicles that ends its task.
    tombstone_vehicles: Vec<Box<dyn vehicle::Vehicle>>,
    // The minimum time granularity is tso. There are tsos per tick.
    tso_per_tick: i64,
    cur_tso: i64,
}


impl Simulator {
    pub fn tick(&mut self, tso: i64) {
        // For every tick, tso can increase by n(n >= 1).

        // Gather all newly pending vehicles, on each road.
        let mut pending_vehicles: HashMap<i32, Vec<Box<dyn vehicle::Vehicle>>>;
        for (_, rt) in self.traffic.roads.iter_mut() {
            let r = rt.handle1(tso);
            for (road_id, v) in r.pending_vehicles.into_iter() {
                match pending_vehicles.entry(road_id) {
                    Vacant(entry) => {
                        entry.insert(vec![v]);
                    },
                    Occupied(mut entry) => {
                        entry.get_mut().push(v)
                    }
                }
            }
        }

        for (road_id, v) in pending_vehicles.into_iter() {
            self.traffic.roads.get_mut(&road_id).unwrap().pending_vehicles.extend(v.into_iter());
        }

        self.cur_tso += self.tso_per_tick;
    }
}


