
use crate::roadmap::Distance;

#[derive(Debug, Clone, Default)]
pub struct Location {
    // road id
    pub road: i32,
    // offset is in [0, road.weight)
    pub offset: Distance,
}

#[derive(Debug, Clone, Default)]
pub struct GotoIntention {
    pub from: Location,
    pub via: Vec<i32>,
    pub to: Location,
}

#[derive(Debug, Clone, Default)]
pub struct BlockedIntention {
    pub next: GotoIntention,
}

pub enum Intention {
    Die,
    Goto(GotoIntention),
    Block((GotoIntention, i32)),
    Blocked(GotoIntention),
    // wrapped GotoIntention, arrived_time is for queueing, remain_distance in this tick round.
    Pending((GotoIntention, f64, Distance))
}

pub trait Vehicle {
    // Get current intention.
    fn intention(&self) -> &Intention;
    fn intention_mut(&mut self) -> &mut Intention;
    // In this tick, we can still go this much distance.
    // If we `go_distance` and `blocked_by`, then `get_left_equivalent_distance` will decrease.
    fn get_left_equivalent_distance(&self) -> Distance;
    fn get_left_equivalent_time(&self) -> f64;
    fn go_distance(&mut self, distance: Distance);
    // Returns true if this car can endure the traffic light in current time piece.
    fn blocked_by(&mut self, delta_tso: i64) -> bool;
    // When a new tick starts, will reset all state.
    fn new_tick(&mut self, total_tsos: i64);
    fn get_id(&self) -> i32;
    // blocked time + running time = time elapsed since tick start
    fn get_blocked_time(&self) -> f64;
    fn get_running_time(&self) -> f64;
}

pub struct EndToEndCar {
    id: i32,
    speed: Distance,
    // Current intention, intention.from means current location
    intention: Intention,
    // Vehicle can be blocked before traffic light for ticks.
    // Ticks will be translated into decreasing distance, to avoid handling float.
    left_equivalent_distance: Distance,
    blocked_time: f64,
    running_time: f64,
}

impl Vehicle for EndToEndCar {
    fn intention(&self) -> & Intention {
        & self.intention
    }
    fn intention_mut(&mut self) -> &mut Intention {
        &mut self.intention
    }
    fn get_left_equivalent_distance(&self) -> Distance {
        self.left_equivalent_distance
    }
    fn get_left_equivalent_time(&self) -> f64 {
        self.left_equivalent_distance as f64 / self.speed as f64
    }
    fn go_distance(&mut self, distance: Distance) {
        self.running_time += distance as f64 / self.speed as f64;
        self.left_equivalent_distance -= distance;
    }
    fn blocked_by(&mut self, delta_tso: i64) -> bool {
        self.blocked_time += delta_tso as f64;
        if self.left_equivalent_distance > delta_tso * self.speed {
            self.left_equivalent_distance -= delta_tso * self.speed;
            true
        } else{
            self.left_equivalent_distance = 0;
            false
        }
    }
    fn new_tick(&mut self, total_tsos: i64) {
        self.left_equivalent_distance = (self.speed * total_tsos) as i64;
    }
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_blocked_time(&self) -> f64 {
        self.blocked_time
    }
    fn get_running_time(&self) -> f64 {
        self.running_time
    }
}