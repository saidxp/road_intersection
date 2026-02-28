mod drawroad;
mod vehicle;
mod traffic_light;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use vehicle::{Vehicle, Direction, Route};
use traffic_light::TrafficLightSystem;
use drawroad::draw_scene;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Traffic Simulation - [Arrows] spawn | [R] random | [Esc] quit", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut traffic_lights = TrafficLightSystem::new();
    let mut last_spawn: std::collections::HashMap<u8, Instant> = std::collections::HashMap::new();
    let spawn_cooldown = Duration::from_millis(800);
    let start_time = Instant::now();

    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::KeyDown { keycode: Some(kc), .. } => {
                    let dir = match kc {
                        Keycode::Up    => Some(Direction::North),
                        Keycode::Down  => Some(Direction::South),
                        Keycode::Right => Some(Direction::East),
                        Keycode::Left  => Some(Direction::West),
                        Keycode::R => {
                            let dirs = [Direction::North, Direction::South, Direction::East, Direction::West];
                            let tick = start_time.elapsed().subsec_millis();
                            Some(dirs[(tick % 4) as usize].clone())
                        }
                        Keycode::Escape => { running = false; None }
                        _ => None,
                    };

                    if let Some(d) = dir {
                        let key: u8 = d.as_u8();
                        let now = Instant::now();
                        let can_spawn = last_spawn.get(&key)
                            .map(|t| now.duration_since(*t) >= spawn_cooldown)
                            .unwrap_or(true);

                        if can_spawn {
                            let safe = vehicles.iter()
                                .filter(|v| v.direction == d)
                                .all(|v| v.distance_from_spawn() > 60.0);

                            if safe {
                                let tick = start_time.elapsed().subsec_millis();
                                let route = Route::random(tick);
                                vehicles.push(Vehicle::new(d.clone(), route));
                                last_spawn.insert(key, now);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        traffic_lights.update(&vehicles);

        for i in 0..vehicles.len() {
            let ahead = find_vehicle_ahead(&vehicles, i);
            vehicles[i].update(&traffic_lights, ahead);
        }

        vehicles.retain(|v| v.is_alive());

        draw_scene(&mut canvas, &vehicles, &traffic_lights);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn find_vehicle_ahead(vehicles: &[Vehicle], idx: usize) -> Option<(f32, f32)> {
    let v = &vehicles[idx];
    let my_dist = v.distance_from_spawn();
    let mut closest: Option<(f32, f32, f32)> = None;

    for (i, other) in vehicles.iter().enumerate() {
        if i == idx || other.direction != v.direction { continue; }
        let other_dist = other.distance_from_spawn();
        if other_dist > my_dist {
            let d = other_dist - my_dist;
            if closest.is_none() || d < closest.unwrap().2 {
                closest = Some((other.x, other.y, d));
            }
        }
    }

    closest.map(|(x, y, _)| (x, y))
}