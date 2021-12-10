use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use crate::roadmap::{RegulatedRoad, ShortestPath, Distance};
use crate::vehicle::{GotoIntention, Intention};
use crate::vehicle;
use crate::roadmap;
use crate::trafficlight;

struct Runner {
    regulated_roadmap: roadmap::RegulatedRoadMap,
    shortest_path: Option<HashMap<(i32, i32), roadmap::ShortestPath>>,
    vehicles: Vec<Box<dyn vehicle::Vehicle>>,
    // The minimum time granularity is tso. There are tsos per tick.
    tso_per_tick: i64,
    cur_tso: i64,
}

impl Runner {
    pub fn tick(&mut self, tso: i64) {
        for v in self.vehicles.iter_mut() {
            v.new_tick(self.tso_per_tick);
        }

        for v in self.vehicles.iter_mut() {
            let intention = v.intention_mut();
            let new_intention = match intention {
                vehicle::Intention::Die => {
                    vehicle::Intention::Die
                },
                vehicle::Intention::Goto(ref mut g) => {

                },
            }
        }

        self.cur_tso += self.tso_per_tick;
    }
}