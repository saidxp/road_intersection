mod app_config;
mod types;
mod entity {
    pub mod vehicle;
    pub mod traffic_light;
}
mod manager {
    pub mod intersection;
    pub mod input;
    pub mod spawner;
}
mod render {
    pub mod draw;
    pub mod palette;
}

use macroquad::prelude::*;
use manager::intersection::IntersectionManager;

fn window_conf() -> Conf {
    Conf {
        window_title: "Clean Road Intersection".to_string(),
        window_width: 800,
        window_height: 800,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut intersection = IntersectionManager::new();

    loop {
        intersection.update();
        intersection.draw();
        next_frame().await
    }
}
