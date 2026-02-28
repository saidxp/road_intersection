mod drawroad;
use sdl2::pixels::Color;
use drawroad::draw_roads_and_lights;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Traffic Simulation", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;

    while running {
        
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                running = false;
            }
        }

        // Call your separated draw function
        draw_roads_and_lights(&mut canvas);
    }
}