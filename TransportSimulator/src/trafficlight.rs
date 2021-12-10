
use crate::roadmap::{RegulatedRoad, ShortestPath, Distance};

pub trait TrafficLight {
    // How long will this car wait if it arrived at tso?
    // return 0 if it can directly pass.
    fn left_time(&self, from: &RegulatedRoad, to: &RegulatedRoad, tso: i64) -> i64;
}

pub struct RoundRobinTrafficLight {
    pub interval: i64,
}

impl TrafficLight for RoundRobinTrafficLight {
    fn left_time(&self, from: &RegulatedRoad, to: &RegulatedRoad, tso: i64) -> i64 {
        let turn = (tso / self.interval) % (to.inbounds.len() as i64);
        0
    }
}