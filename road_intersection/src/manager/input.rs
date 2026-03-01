use macroquad::prelude::*;
use crate::entity::vehicle::Vehicle;
use crate::types::Origin;
use crate::app_config::*;
use crate::manager::spawner::Spawner;
use std::process::exit;

pub struct InputManager;

impl InputManager {
    pub fn handle_input(vehicles: &mut Vec<Vehicle>, center: (f32, f32)) {
        let n_c = vehicles.iter().filter(|v| v.origin == Origin::North && !v.turned).count();
        let s_c = vehicles.iter().filter(|v| v.origin == Origin::South && !v.turned).count();
        let e_c = vehicles.iter().filter(|v| v.origin == Origin::East && !v.turned).count();
        let w_c = vehicles.iter().filter(|v| v.origin == Origin::West && !v.turned).count();
        
        let counts = [n_c, s_c, e_c, w_c]; 
        let s = CAR_SPEED;
        let spawn_offset = 25.0;

        if is_key_pressed(KeyCode::Up) && counts[1] < LANE_CAPACITY {
             Spawner::try_spawn(vehicles, (center.0 + spawn_offset, screen_height() + 20.0), Origin::South, (0.0, -s));
        }
        if is_key_pressed(KeyCode::Down) && counts[0] < LANE_CAPACITY {
             Spawner::try_spawn(vehicles, (center.0 - spawn_offset, -20.0), Origin::North, (0.0, s));
        }
        if is_key_pressed(KeyCode::Right) && counts[2] < LANE_CAPACITY {
             Spawner::try_spawn(vehicles, (-20.0, center.1 + spawn_offset), Origin::East, (s, 0.0));
        }
        if is_key_pressed(KeyCode::Left) && counts[3] < LANE_CAPACITY {
             Spawner::try_spawn(vehicles, (screen_width() + 20.0, center.1 - spawn_offset), Origin::West, (-s, 0.0));
        }
        
        if is_key_pressed(KeyCode::R) {
            let random_dir = rand::gen_range(0, 4);
            match random_dir {
                0 if counts[0] < LANE_CAPACITY => {
                    Spawner::try_spawn(vehicles, (center.0 - spawn_offset, -20.0), Origin::North, (0.0, s));
                }
                1 if counts[1] < LANE_CAPACITY => {
                    Spawner::try_spawn(vehicles, (center.0 + spawn_offset, screen_height() + 20.0), Origin::South, (0.0, -s));
                }
                2 if counts[2] < LANE_CAPACITY => {
                    Spawner::try_spawn(vehicles, (-20.0, center.1 + spawn_offset), Origin::East, (s, 0.0));
                }
                3 if counts[3] < LANE_CAPACITY => {
                    Spawner::try_spawn(vehicles, (screen_width() + 20.0, center.1 - spawn_offset), Origin::West, (-s, 0.0));
                }
                _ => {}
            }
        }

        if is_key_pressed(KeyCode::Escape){
            exit(0)
        }
    }
}
