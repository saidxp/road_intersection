use macroquad::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::app_config::*;
use crate::types::{Origin, Route};

static VEHICLE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: usize,
    pub pos: (f32, f32),
    pub speed: (f32, f32),
    pub origin: Origin,
    pub route: Route,
    pub color: Color,
    pub turned: bool,
}

impl Vehicle {
    pub fn new(pos: (f32, f32), speed: (f32, f32), origin: Origin, route: Route, color: Color) -> Self {
        Self {
            id: VEHICLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            pos,
            speed,
            origin,
            route,
            color,
            turned: false,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.0 - CAR_SIZE / 2.0,
            self.pos.1 - CAR_SIZE / 2.0,
            CAR_SIZE,
            CAR_SIZE,
            self.color,
        );

        draw_rectangle_lines(
            self.pos.0 - CAR_SIZE / 2.0,
            self.pos.1 - CAR_SIZE / 2.0,
            CAR_SIZE,
            CAR_SIZE,
            2.0,
            Color::new(0.0, 0.0, 0.0, 0.2),
        );
    }

    pub fn update(&mut self, center: (f32, f32)) {
        self.pos.0 += self.speed.0;
        self.pos.1 += self.speed.1;
        
        if !self.turned {
            self.update_turning_direction(center);
        }
    }

    pub fn should_despawn(&self) -> bool {
        self.pos.0 < -DESPAWN_OFFSET
            || self.pos.0 > screen_width() + DESPAWN_OFFSET
            || self.pos.1 < -DESPAWN_OFFSET
            || self.pos.1 > screen_height() + DESPAWN_OFFSET
    }

    fn update_turning_direction(&mut self, center: (f32, f32)) {
        let s = CAR_SPEED;
        match self.route {
            Route::Straight => {
                let passed = match self.origin {
                    Origin::South => self.pos.1 < center.1 - LANE_OFFSET,
                    Origin::North => self.pos.1 > center.1 + LANE_OFFSET,
                    Origin::East => self.pos.0 > center.0 + LANE_OFFSET,
                    Origin::West => self.pos.0 < center.0 - LANE_OFFSET,
                };
                if passed {
                    self.turned = true;
                }
            }
            Route::Right => {
                let turn_p = match self.origin {
                    Origin::South => self.pos.1 < center.1 + CAR_SIZE,
                    Origin::North => self.pos.1 > center.1 - CAR_SIZE,
                    Origin::East => self.pos.0 > center.0 - CAR_SIZE,
                    Origin::West => self.pos.0 < center.0 + CAR_SIZE,
                };
                if turn_p {
                    self.speed = match self.origin {
                        Origin::South => (s, 0.0),
                        Origin::North => (-s, 0.0),
                        Origin::East => (0.0, s),
                        Origin::West => (0.0, -s),
                    };
                    self.turned = true;
                }
            }
            Route::Left => {
                let turn_p = match self.origin {
                    Origin::South => self.pos.1 < center.1 - CAR_SIZE,
                    Origin::North => self.pos.1 > center.1 + CAR_SIZE,
                    Origin::East => self.pos.0 > center.0 + CAR_SIZE,
                    Origin::West => self.pos.0 < center.0 - CAR_SIZE,
                };
                if turn_p {
                    self.speed = match self.origin {
                        Origin::South => (-s, 0.0),
                        Origin::North => (s, 0.0),
                        Origin::East => (0.0, -s),
                        Origin::West => (0.0, s),
                    };
                    self.turned = true;
                }
            }
        }
    }

    pub fn is_safe_to_move(&self, vehicles: &[Vehicle], active_green: Option<Origin>, center: (f32, f32)) -> bool {
        let mut can_move = true;

        if !self.turned {
            let is_at_stop = match self.origin {
                Origin::South => {
                    self.pos.1 > center.1 + LANE_OFFSET && self.pos.1 < center.1 + (LANE_OFFSET + CAR_SIZE * 1.5)
                }
                Origin::North => {
                    self.pos.1 < center.1 - LANE_OFFSET && self.pos.1 > center.1 - (LANE_OFFSET + CAR_SIZE * 1.5)
                }
                Origin::East => {
                    self.pos.0 < center.0 - LANE_OFFSET && self.pos.0 > center.0 - (LANE_OFFSET + CAR_SIZE * 1.5)
                }
                Origin::West => {
                    self.pos.0 > center.0 + LANE_OFFSET && self.pos.0 < center.0 + (LANE_OFFSET + CAR_SIZE * 1.5)
                }
            };

            if is_at_stop && Some(self.origin) != active_green {
                can_move = false;
            }
        }

        if can_move {
            for other in vehicles {
                if self.id == other.id {
                    continue;
                }
                
                let dx = self.pos.0 - other.pos.0;
                let dy = self.pos.1 - other.pos.1;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < AHEAD_CHECK_DISTANCE * AHEAD_CHECK_DISTANCE &&
                   self.is_ahead(other.pos)
                {
                    can_move = false;
                    break;
                }
            }
        }

        can_move
    }

    fn is_ahead(&self, other: (f32, f32)) -> bool {
        let epsilon = 0.1;
        
        if self.speed.0 > epsilon { 
            return other.0 > self.pos.0 && (other.1 - self.pos.1).abs() < AHEAD_CHECK_LATERAL; 
        }
        if self.speed.0 < -epsilon { 
            return other.0 < self.pos.0 && (other.1 - self.pos.1).abs() < AHEAD_CHECK_LATERAL; 
        }
        if self.speed.1 > epsilon { 
            return other.1 > self.pos.1 && (other.0 - self.pos.0).abs() < AHEAD_CHECK_LATERAL; 
        }
        if self.speed.1 < -epsilon { 
            return other.1 < self.pos.1 && (other.0 - self.pos.0).abs() < AHEAD_CHECK_LATERAL; 
        }
        false
    }
}
