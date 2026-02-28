use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads_and_lights(canvas: &mut Canvas<Window>) {
    
    canvas.set_draw_color(Color::RGB(0, 150, 0)); 
    canvas.clear();

    let (w, h) = canvas.output_size().unwrap();
    let center_x = w / 2;
    let center_y = h / 2;
    let road_width = 120;

    // Vertical road
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.fill_rect(Rect::new(center_x as i32 - road_width / 2, 0, road_width as u32, h)).unwrap();

    // Horizontal road
    canvas.fill_rect(Rect::new(0, center_y as i32 - road_width / 2, w, road_width as u32)).unwrap();

    // Traffic lights
    let light_size = 20;
    canvas.set_draw_color(Color::RGB(255, 0, 0)); // red

    // North
    canvas.fill_rect(Rect::new(center_x as i32 - 10, center_y as i32 - road_width / 2 - 40, light_size, light_size)).unwrap();
    // South
    canvas.fill_rect(Rect::new(center_x as i32 - 10, center_y as i32 + road_width / 2 + 20, light_size, light_size)).unwrap();
    // West
    canvas.fill_rect(Rect::new(center_x as i32 - road_width / 2 - 40, center_y as i32 - 10, light_size, light_size)).unwrap();
    // East
    canvas.fill_rect(Rect::new(center_x as i32 + road_width / 2 + 20, center_y as i32 - 10, light_size, light_size)).unwrap();

    // Present frame
    canvas.present();
}