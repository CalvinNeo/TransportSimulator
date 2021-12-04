
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
    fn elaspe_distance(&mut self, delta_tso: i64);
    fn new_tick(&mut self, tso: i64);
    fn get_id(&self) -> i32;
}

pub struct EndToEndCar {
    id: i32,
    speed: Distance,
    // Current intention, intention.from means current location
    intention: Intention,
    // Vehicle can be blocked before traffic light for ticks.
    // Ticks will be translated into decreasing distance, to avoid handling float.
    left_equivalent_distance: Distance,
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
        self.left_equivalent_distance -= distance;
    }
    fn elaspe_distance(&mut self, delta_tso: i64) {
        self.left_equivalent_distance -= delta_tso * self.speed;
    }
    fn new_tick(&mut self, total_tsos: i64) {
        self.left_equivalent_distance = (self.speed * total_tsos) as i64;
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}