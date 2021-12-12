use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use crate::roadmap::{RegulatedRoad, ShortestPath, Distance};
use crate::vehicle::{BlockedIntention, GotoIntention, Intention, Location};
use crate::vehicle;
use crate::roadmap;
use crate::trafficlight;
use crate::trafficlight::TrafficLight;

struct Runner {
    regulated_roadmap: roadmap::RegulatedRoadMap,
    shortest_path: Option<HashMap<(i32, i32), roadmap::ShortestPath>>,
    vehicles: Vec<Box<dyn vehicle::Vehicle>>,
    pending_vehicles: HashMap<i32, Box<dyn vehicle::Vehicle>>,
    traffic_light: Box<dyn TrafficLight>,
    // The minimum time granularity is tso. There are tsos per tick.
    tso_per_tick: i64,
    cur_tso: i64,
}

impl Runner {
    // Handle one vehicle, until it consumed all time piece, or pending at some traffic light.
    pub fn handle_vehicle(&mut self, mut vv: Box<dyn vehicle::Vehicle>) {
        let new_intention = loop {
            let v = &mut vv;
            let intention = v.intention();
            let (new_intention, finished) = match intention {
                Intention::Die => {
                    (vehicle::Intention::Die, true)
                },
                Intention::Goto(gg) => {
                    let mut g = gg.clone();
                    let ability = v.get_left_equivalent_distance();
                    let road = self.regulated_roadmap.roads.get(&g.from.road).unwrap();
                    let prev_road = g.from.road;
                    if ability >= road.weight - g.from.offset {
                        // We can go to next road, not finishing tick.
                        let next = if g.via.is_empty() {
                            if g.from.road == g.to.road {
                                // Shall die
                                None
                            }else{
                                g.from = g.to.clone();
                                g.from.offset = 0;
                                Some(g)
                            }
                        } else {
                            let r = g.via.pop().unwrap();
                            g.from = Location {
                                road: r,
                                offset: 0,
                            };
                            Some(g)
                        };
                        if next.is_none() {
                            (Intention::Die, true)
                        } else {
                            (Intention::Block((next.unwrap(), prev_road)), false)
                        }
                    } else {
                        // We still run in this road, and finish tick.
                        g.from.offset += ability.clone();
                        v.go_distance(ability);
                        (Intention::Goto(g), true)
                    }
                },
                Intention::Block((gg, prev_road)) => {
                    let mut g = gg.clone();
                    // The vehicle is in front of traffic light.
                    let from = self.regulated_roadmap.roads.get(prev_road).unwrap();
                    let to = self.regulated_roadmap.roads.get(&g.from.road).unwrap();
                    let wait_time = self.traffic_light.wait_time(from, to, v.get_left_equivalent_time());
                    let arrived_time = self.tso_per_tick as f64 - v.get_left_equivalent_time();
                    if wait_time == 0 {
                        // No wait, just pass
                        (Intention::Goto(g), false)
                    } else {
                        if wait_time >= v.get_left_equivalent_distance() {
                            // If we blocked here.
                            (Intention::Block((g, prev_road.to_owned())), true)
                        } else {
                            // If we can still go after block.
                            v.go_distance(wait_time);
                            let remain = wait_time - v.get_left_equivalent_distance();
                            (Intention::Pending((g, arrived_time, remain)), true)
                        }
                    }
                },
                _ => panic!()
            };
            if finished {
                break new_intention;
            }
        };
        match new_intention {
            Intention::Pending((g, arrived_time, remain)) => {
                self.pending_vehicles.insert(g.from.road, vv);
            },
            _ => {
                self.vehicles.push(vv);
            },
        }
    }
    pub fn tick(&mut self, tso: i64) {
        for v in self.vehicles.iter_mut() {
            v.new_tick(self.tso_per_tick);
        }

        // This is one iteration. Iteration will ends if all vehicles spends all time pieces.
        let mut vehicles = std::mem::take(&mut self.vehicles);
        for v in vehicles.into_iter() {
            // Option<Box<dyn vehicle::Vehicle>>
            self.handle_vehicle(v);
        }

        self.cur_tso += self.tso_per_tick;
    }
}