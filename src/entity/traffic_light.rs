use macroquad::prelude::*;
use std::collections::HashMap;
use crate::types::Origin;
use crate::entity::vehicle::Vehicle;
use crate::app_config::*;
use crate::render::palette::*;

pub struct TrafficLightSystem {
    active_green: Option<Origin>,
    green_timer: f32,
}

impl TrafficLightSystem {
    pub fn new() -> Self {
        Self {
            active_green: None,
            green_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, vehicles: &[Vehicle], center: (f32, f32)) {
        self.green_timer += dt;

        let center_count = vehicles
            .iter()
            .filter(|v| {
                (v.pos.0 - center.0).abs() < CENTER_HALF && (v.pos.1 - center.1).abs() < CENTER_HALF
            })
            .count();
        
        let center_empty = center_count == 0;

        let should_switch = match self.active_green {
            None => true,
            Some(_) => center_empty && self.green_timer >= MIN_GREEN_TIME,
        };

        if should_switch {
            self.decide_next_green(vehicles);
        }
    }

    fn decide_next_green(&mut self, vehicles: &[Vehicle]) {
        let n_c = vehicles.iter().filter(|v| v.origin == Origin::North && !v.turned).count();
        let s_c = vehicles.iter().filter(|v| v.origin == Origin::South && !v.turned).count();
        let e_c = vehicles.iter().filter(|v| v.origin == Origin::East && !v.turned).count();
        let w_c = vehicles.iter().filter(|v| v.origin == Origin::West && !v.turned).count();

        let ratios = HashMap::from([
            (Origin::North, n_c as f32),
            (Origin::South, s_c as f32),
            (Origin::East, e_c as f32),
            (Origin::West, w_c as f32),
        ]);

        let mut best_lane = None;
        let mut best_score = -1.0;

        for lane in [Origin::North, Origin::South, Origin::East, Origin::West] {
            let score = ratios[&lane];
            
            if score > best_score && score > 0.0 {
                best_score = score;
                best_lane = Some(lane);
            }
        }

        if let Some(lane) = best_lane {
             if self.active_green != Some(lane) {
                self.active_green = Some(lane);
                self.green_timer = 0.0;
             }
        } else if self.active_green.is_none() {
             self.active_green = Some(Origin::North);
        }
    }
    
    pub fn get_active_green(&self) -> Option<Origin> {
        self.active_green
    }

    pub fn draw(&self, center: (f32, f32)) {
         let get_color = |o: Origin| {
             if self.active_green == Some(o) { COLOR_LIGHT_GO } else { COLOR_LIGHT_STOP }
         };

         let padding = LIGHT_PADDING;
         let radius = LIGHT_SIZE / 2.0;
         let full_size = LIGHT_SIZE + padding * 2.0;

         let draw_light = |x: f32, y: f32, color: Color| {
            draw_rectangle(x, y, full_size, full_size, COLOR_LIGHT_HOUSING);
            
            let cx = x + padding + radius;
            let cy = y + padding + radius;
            
            draw_circle(cx, cy, radius, color);
         };

         draw_light(center.0 - LIGHT_OFFSET - padding, center.1 - LIGHT_OFFSET - padding, get_color(Origin::North));
         draw_light(center.0 + LANE_OFFSET - padding, center.1 + LANE_OFFSET - padding, get_color(Origin::South));
         draw_light(center.0 - LIGHT_OFFSET - padding, center.1 + LANE_OFFSET - padding, get_color(Origin::East));
         draw_light(center.0 + LANE_OFFSET - padding, center.1 - LIGHT_OFFSET - padding, get_color(Origin::West));
    }
}
