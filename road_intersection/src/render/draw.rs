use macroquad::prelude::*;
use crate::app_config::*;
use crate::render::palette::*;

pub struct Renderer;

impl Renderer {
    pub fn draw_environment(center: (f32, f32)) {
        let (w, h) = (screen_width(), screen_height());
        
        clear_background(COLOR_GRASS);

        draw_rectangle(
            center.0 - ROAD_WIDTH / 2.0,
            0.0,
            ROAD_WIDTH,
            h,
            COLOR_ASPHALT,
        );
        draw_rectangle(
            0.0,
            center.1 - ROAD_WIDTH / 2.0,
            w,
            ROAD_WIDTH,
            COLOR_ASPHALT,
        );
        
        draw_rectangle(
            center.0 - ROAD_WIDTH / 2.0,
            center.1 - ROAD_WIDTH / 2.0,
            ROAD_WIDTH,
            ROAD_WIDTH,
            COLOR_ASPHALT,
        );

        let dash_length = 20.0;
        let dash_gap = 20.0;

        draw_line(center.0 - 2.0, 0.0, center.0 - 2.0, center.1 - ROAD_WIDTH / 2.0, 2.0, COLOR_MARKING_YELLOW);
        draw_line(center.0 + 2.0, 0.0, center.0 + 2.0, center.1 - ROAD_WIDTH / 2.0, 2.0, COLOR_MARKING_YELLOW);
        
        draw_line(center.0 - 2.0, center.1 + ROAD_WIDTH / 2.0, center.0 - 2.0, h, 2.0, COLOR_MARKING_YELLOW);
        draw_line(center.0 + 2.0, center.1 + ROAD_WIDTH / 2.0, center.0 + 2.0, h, 2.0, COLOR_MARKING_YELLOW);

        draw_line(0.0, center.1 - 2.0, center.0 - ROAD_WIDTH / 2.0, center.1 - 2.0, 2.0, COLOR_MARKING_YELLOW);
        draw_line(0.0, center.1 + 2.0, center.0 - ROAD_WIDTH / 2.0, center.1 + 2.0, 2.0, COLOR_MARKING_YELLOW);

        draw_line(center.0 + ROAD_WIDTH / 2.0, center.1 - 2.0, w, center.1 - 2.0, 2.0, COLOR_MARKING_YELLOW);
        draw_line(center.0 + ROAD_WIDTH / 2.0, center.1 + 2.0, w, center.1 + 2.0, 2.0, COLOR_MARKING_YELLOW);

        let draw_dashed_line = |x1: f32, y1: f32, x2: f32, y2: f32| {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let len = (dx*dx + dy*dy).sqrt();
            let angle = dy.atan2(dx);
            let steps = (len / (dash_length + dash_gap)) as i32;
            
            for i in 0..steps {
                let start_dist = i as f32 * (dash_length + dash_gap);
                let end_dist = start_dist + dash_length;
                if start_dist > len { break; }
                
                let sx = x1 + start_dist * angle.cos();
                let sy = y1 + start_dist * angle.sin();
                let ex = x1 + end_dist.min(len) * angle.cos();
                let ey = y1 + end_dist.min(len) * angle.sin();
                
                draw_line(sx, sy, ex, ey, 2.0, COLOR_MARKING_WHITE);
            }
        };

        draw_dashed_line(center.0 - LANE_OFFSET, 0.0, center.0 - LANE_OFFSET, center.1 - ROAD_WIDTH / 2.0);
        draw_dashed_line(center.0 + LANE_OFFSET, 0.0, center.0 + LANE_OFFSET, center.1 - ROAD_WIDTH / 2.0);
        draw_dashed_line(center.0 - LANE_OFFSET, center.1 + ROAD_WIDTH / 2.0, center.0 - LANE_OFFSET, h);
        draw_dashed_line(center.0 + LANE_OFFSET, center.1 + ROAD_WIDTH / 2.0, center.0 + LANE_OFFSET, h);

        draw_dashed_line(0.0, center.1 - LANE_OFFSET, center.0 - ROAD_WIDTH / 2.0, center.1 - LANE_OFFSET);
        draw_dashed_line(0.0, center.1 + LANE_OFFSET, center.0 - ROAD_WIDTH / 2.0, center.1 + LANE_OFFSET);
        draw_dashed_line(center.0 + ROAD_WIDTH / 2.0, center.1 - LANE_OFFSET, w, center.1 - LANE_OFFSET);
        draw_dashed_line(center.0 + ROAD_WIDTH / 2.0, center.1 + LANE_OFFSET, w, center.1 + LANE_OFFSET);

        let stop_width = 4.0;
        draw_line(center.0, center.1 - ROAD_WIDTH / 2.0, center.0 - ROAD_WIDTH / 2.0, center.1 - ROAD_WIDTH / 2.0, stop_width, COLOR_MARKING_WHITE);
        draw_line(center.0, center.1 + ROAD_WIDTH / 2.0, center.0 + ROAD_WIDTH / 2.0, center.1 + ROAD_WIDTH / 2.0, stop_width, COLOR_MARKING_WHITE);
        draw_line(center.0 - ROAD_WIDTH / 2.0, center.1, center.0 - ROAD_WIDTH / 2.0, center.1 + ROAD_WIDTH / 2.0, stop_width, COLOR_MARKING_WHITE);
        draw_line(center.0 + ROAD_WIDTH / 2.0, center.1, center.0 + ROAD_WIDTH / 2.0, center.1 - ROAD_WIDTH / 2.0, stop_width, COLOR_MARKING_WHITE);
    }
}
