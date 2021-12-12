
use crate::roadmap::{RegulatedRoad, ShortestPath, Distance};

pub trait TrafficLight {
    // How long will this car wait if it arrived at tso?
    // return 0 if it can directly pass.
    fn wait_time(&self, from: &RegulatedRoad, to: &RegulatedRoad, tso: f64) -> Distance;
}

pub struct RoundRobinTrafficLight {
    pub interval: i64,
}

impl TrafficLight for RoundRobinTrafficLight {
    fn wait_time(&self, from: &RegulatedRoad, to: &RegulatedRoad, tso: f64) -> Distance {
        // let turn = (tso / self.interval as f64) % (to.inbounds.len() as i64);
        0
    }
}