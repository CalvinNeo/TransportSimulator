
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

// For every vehicle in `running_vehicles`, if a vehicle is fast enough to travel through multiple roads in one tick,
// it should also be added into other pending lists.
// By this, we can exactly know which vehicles can pass traffic light,
// and study interaction(like traffic jam) between those vehicles.
// However, due to unpredictable traffic light, and vehicle interaction, vehicles may not actually present.
// As a result, we can find some road's `pending_vehicle` is dependent on other roads.
pub enum Intention {
    Die,
    Goto(GotoIntention),
}

pub trait Vehicle {
    // Get current intention.
    fn intention(&self) -> &Intention;
    fn intention_mut(&mut self) -> &mut Intention;
    // In this tick, we can go this much distance.
    fn get_left_equivalent_distance(&self) -> Distance;
    fn go_distance(&mut self, distance: Distance);
    fn blocked_by(&mut self, delta_tso: i64);
    fn new_tick(&mut self, tso: i64);
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
    fn go_distance(&mut self, distance: Distance) {
        self.running_time += distance as f64 / self.speed as f64;
        self.left_equivalent_distance -= distance;
    }
    fn blocked_by(&mut self, delta_tso: i64) {
        self.blocked_time += delta_tso as f64;
        self.left_equivalent_distance -= delta_tso * self.speed;
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