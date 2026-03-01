use macroquad::prelude::*;
use crate::app_config::*;
use crate::entity::vehicle::Vehicle;
use crate::types::{Origin, Route};
use crate::render::palette::*;

pub struct Spawner;

impl Spawner {
    pub fn try_spawn(vehicles: &mut Vec<Vehicle>, pos: (f32, f32), origin: Origin, speed: (f32, f32)) {
        let is_safe = !vehicles.iter().any(|v| {
            let dx = v.pos.0 - pos.0;
            let dy = v.pos.1 - pos.1;
            (dx * dx + dy * dy) < SPAWN_DISTANCE_CHECK * SPAWN_DISTANCE_CHECK
        });

        if is_safe {
            let (route, color) = Self::get_random_attributes();
            vehicles.push(Vehicle::new(pos, speed, origin, route, color));
        }
    }

    fn get_random_attributes() -> (Route, Color) {
        let route = match rand::gen_range(0, 3) {
            0 => Route::Right,
            1 => Route::Left,
            _ => Route::Straight,
        };

        let color_idx = rand::gen_range(0, CAR_COLORS.len());
        let color = CAR_COLORS[color_idx];

        (route, color)
    }
}
