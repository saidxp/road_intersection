use std::time::{Duration, Instant};
use crate::vehicle::{Vehicle, Direction};

#[derive(Clone, PartialEq)]
pub enum Phase {
    NorthSouth,
    EastWest,
}

#[derive(Clone)]
pub struct TrafficLightSystem {
    pub phase: Phase,
    last_switch: Instant,
}

impl TrafficLightSystem {
    pub fn new() -> Self {
        TrafficLightSystem {
            phase: Phase::NorthSouth,
            last_switch: Instant::now(),
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        let ns_waiting = vehicles.iter().filter(|v| {
            (v.direction == Direction::North || v.direction == Direction::South) && v.waiting
        }).count();
        let ew_waiting = vehicles.iter().filter(|v| {
            (v.direction == Direction::East || v.direction == Direction::West) && v.waiting
        }).count();

        let elapsed = self.last_switch.elapsed();

        let current_congested = match self.phase {
            Phase::NorthSouth => ns_waiting >= 6,
            Phase::EastWest   => ew_waiting >= 6,
        };

        let max_green = if current_congested {
            Duration::from_secs(10)
        } else {
            Duration::from_secs(5)
        };

        let other_has_more = match self.phase {
            Phase::NorthSouth => ew_waiting > ns_waiting,
            Phase::EastWest   => ns_waiting > ew_waiting,
        };

        let should_switch = elapsed >= max_green
            || (elapsed >= Duration::from_secs(3) && other_has_more);

        if should_switch {
            self.phase = match self.phase {
                Phase::NorthSouth => Phase::EastWest,
                Phase::EastWest   => Phase::NorthSouth,
            };
            self.last_switch = Instant::now();
        }
    }

    pub fn is_green(&self, dir: &Direction) -> bool {
        match dir {
            Direction::North | Direction::South => self.phase == Phase::NorthSouth,
            Direction::East  | Direction::West  => self.phase == Phase::EastWest,
        }
    }
}