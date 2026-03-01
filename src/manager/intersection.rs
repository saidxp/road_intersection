use macroquad::prelude::*;
use crate::entity::vehicle::Vehicle;
use crate::entity::traffic_light::TrafficLightSystem;
use crate::manager::input::InputManager;
use crate::render::draw::Renderer;

pub struct IntersectionManager {
    vehicles: Vec<Vehicle>,
    traffic_system: TrafficLightSystem,
    center: (f32, f32),
}

impl IntersectionManager {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::with_capacity(50),
            traffic_system: TrafficLightSystem::new(),
            center: (screen_width() / 2.0, screen_height() / 2.0),
        }
    }

    pub fn update(&mut self) {
        self.center = (screen_width() / 2.0, screen_height() / 2.0);
        let dt = get_frame_time();

        InputManager::handle_input(&mut self.vehicles, self.center);
        
        self.traffic_system.update(dt, &self.vehicles, self.center);
        let active_green = self.traffic_system.get_active_green();

        let mut i = 0;
        while i < self.vehicles.len() {
            if self.vehicles[i].should_despawn() {
                self.vehicles.swap_remove(i);
            } else {
                let safe = self.vehicles[i].is_safe_to_move(&self.vehicles, active_green, self.center);
                
                if safe {
                    self.vehicles[i].update(self.center);
                }
                
                i += 1;
            }
        }
    }

    pub fn draw(&self) {
        Renderer::draw_environment(self.center);

        self.traffic_system.draw(self.center);

        for vehicle in &self.vehicles {
            vehicle.draw();
        }
        
        draw_text(
            &format!("Vehicles: {}", self.vehicles.len()), 
            20.0, 
            30.0, 
            30.0, 
            WHITE
        );
        
        draw_text(
            "Controls: Arrows to spawn, R for random, ESC to quit", 
            20.0, 
            screen_height() - 20.0, 
            20.0, 
            LIGHTGRAY
        );
    }
}
