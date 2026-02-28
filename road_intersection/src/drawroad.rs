use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::vehicle::{Vehicle, CENTER_X, CENTER_Y, ROAD_HALF};
use crate::traffic_light::{TrafficLightSystem, Phase};

pub fn draw_scene(canvas: &mut Canvas<Window>, vehicles: &[Vehicle], lights: &TrafficLightSystem) {
    let (w, h) = canvas.output_size().unwrap();
    let cx = CENTER_X as i32;
    let cy = CENTER_Y as i32;
    let rh = ROAD_HALF as i32;
    let road_w = (ROAD_HALF * 2.0) as u32;

    canvas.set_draw_color(Color::RGB(34, 139, 34));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(60, 60, 60));
    canvas.fill_rect(Rect::new(cx - rh, 0, road_w, h)).unwrap();
    canvas.fill_rect(Rect::new(0, cy - rh, w, road_w)).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut y = 0i32;
    while y < cy - rh {
        canvas.fill_rect(Rect::new(cx - 2, y, 4, 20)).unwrap();
        y += 40;
    }
    y = cy + rh;
    while y < h as i32 {
        canvas.fill_rect(Rect::new(cx - 2, y, 4, 20)).unwrap();
        y += 40;
    }
    let mut x = 0i32;
    while x < cx - rh {
        canvas.fill_rect(Rect::new(x, cy - 2, 20, 4)).unwrap();
        x += 40;
    }
    x = cx + rh;
    while x < w as i32 {
        canvas.fill_rect(Rect::new(x, cy - 2, 20, 4)).unwrap();
        x += 40;
    }

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(Rect::new(cx, cy + rh, rh as u32, 3)).unwrap();
    canvas.fill_rect(Rect::new(cx - rh, cy - rh - 3, rh as u32, 3)).unwrap();
    canvas.fill_rect(Rect::new(cx - rh - 3, cy, 3, rh as u32)).unwrap();
    canvas.fill_rect(Rect::new(cx + rh, cy - rh, 3, rh as u32)).unwrap();

    let light_size: u32 = 18;
    let ls = light_size as i32;
    let offset = 14i32;

    let ns_green = lights.phase == Phase::NorthSouth;
    let ew_green = lights.phase == Phase::EastWest;

    let draw_light = |canvas: &mut Canvas<Window>, x: i32, y: i32, green: bool| {
        
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.fill_rect(Rect::new(x - 2, y - 2, light_size + 4, light_size + 4)).unwrap();
        if green {
            canvas.set_draw_color(Color::RGB(0, 220, 0));
        } else {
            canvas.set_draw_color(Color::RGB(220, 0, 0));
        }
        canvas.fill_rect(Rect::new(x, y, light_size, light_size)).unwrap();
    };

    draw_light(canvas, cx + offset, cy + rh + 8, ns_green);
    draw_light(canvas, cx - rh - ls - 8, cy - rh - ls - 8, ns_green);
    draw_light(canvas, cx - rh - ls - 8, cy + offset, ew_green);
    draw_light(canvas, cx + rh + 8, cy - rh - ls - 8, ew_green);

    for v in vehicles {
        let (r, g, b) = v.route.color();
        canvas.set_draw_color(Color::RGB(r, g, b));
        let vw = v.width() as u32;
        let vh = v.height() as u32;
        canvas.fill_rect(Rect::new(
            v.x as i32 - vw as i32 / 2,
            v.y as i32 - vh as i32 / 2,
            vw, vh
        )).unwrap();

        canvas.set_draw_color(Color::RGB(180, 220, 255));
        let ws = 6u32;
        let wx = v.x as i32 - ws as i32 / 2;
        let wy = v.y as i32 - ws as i32 / 2;
        canvas.fill_rect(Rect::new(wx, wy, ws, ws)).unwrap();
    }

    let legend_items = [
        ("Straight", 255u8, 255u8, 80u8),
        ("Left turn", 80u8, 180u8, 255u8),
        ("Right turn", 255u8, 140u8, 0u8),
    ];
    for (i, (_label, r, g, b)) in legend_items.iter().enumerate() {
        let lx = 10i32;
        let ly = 10i32 + i as i32 * 22;
        canvas.set_draw_color(Color::RGB(*r, *g, *b));
        canvas.fill_rect(Rect::new(lx, ly, 18, 14)).unwrap();
    }

    canvas.present();
}