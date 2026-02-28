use crate::traffic_light::TrafficLightSystem;

pub const WINDOW_W: f32 = 800.0;
pub const WINDOW_H: f32 = 600.0;
pub const CENTER_X: f32 = 400.0;
pub const CENTER_Y: f32 = 300.0;
pub const ROAD_HALF: f32 = 60.0; 
pub const LANE_OFFSET: f32 = 30.0;
pub const VEHICLE_SPEED: f32 = 2.5;
pub const SAFE_DIST: f32 = 50.0; 
pub const VEHICLE_W: f32 = 20.0;
pub const VEHICLE_H: f32 = 34.0;

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    North, 
    South, 
    East, 
    West,  
}

impl Direction {
    pub fn as_u8(&self) -> u8 {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Straight,
    TurnLeft,
    TurnRight,
}

impl Route {
    pub fn random(tick: u32) -> Route {
        match tick % 3 {
            0 => Route::Straight,
            1 => Route::TurnLeft,
            _ => Route::TurnRight,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Route::Straight => (255, 255, 80),   
            Route::TurnLeft => (80, 180, 255),    
            Route::TurnRight => (255, 140, 0),    
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum VehicleState {
    Approaching,
    InIntersection,
    Exiting,
    Done,
}

pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub route: Route,
    pub state: VehicleState,
    pub waiting: bool,
    turn_progress: f32,
    turn_start_x: f32,
    turn_start_y: f32,
    turn_end_x: f32,
    turn_end_y: f32,
    exit_dir: Direction,
}

impl Vehicle {
    pub fn new(dir: Direction, route: Route) -> Self {
        let (x, y) = spawn_pos(&dir);
        Vehicle {
            x, y, direction: dir, route, state: VehicleState::Approaching,
            waiting: false, turn_progress: 0.0,
            turn_start_x: 0.0, turn_start_y: 0.0,
            turn_end_x: 0.0, turn_end_y: 0.0,
            exit_dir: Direction::North,
        }
    }

    pub fn distance_from_spawn(&self) -> f32 {
        let (sx, sy) = spawn_pos(&self.direction);
        ((self.x - sx).powi(2) + (self.y - sy).powi(2)).sqrt()
    }

    pub fn is_alive(&self) -> bool {
        self.state != VehicleState::Done
    }

    pub fn update(&mut self, lights: &TrafficLightSystem, vehicle_ahead: Option<(f32, f32)>) {
        match self.state.clone() {
            VehicleState::Approaching => self.update_approaching(lights, vehicle_ahead),
            VehicleState::InIntersection => self.update_in_intersection(),
            VehicleState::Exiting => self.update_exiting(),
            VehicleState::Done => {}
        }
    }

    fn update_approaching(&mut self, lights: &TrafficLightSystem, ahead: Option<(f32, f32)>) {
        let stop_line = stop_line_pos(&self.direction);

        let at_stop = match self.direction {
            Direction::North => self.y <= stop_line + VEHICLE_H,
            Direction::South => self.y >= stop_line - VEHICLE_H,
            Direction::East  => self.x >= stop_line - VEHICLE_H,
            Direction::West  => self.x <= stop_line + VEHICLE_H,
        };

        let blocked_by_ahead = if let Some((ax, ay)) = ahead {
            let dist = ((self.x - ax).powi(2) + (self.y - ay).powi(2)).sqrt();
            dist < SAFE_DIST
        } else {
            false
        };

        let green = lights.is_green(&self.direction);

        let should_stop = blocked_by_ahead || (at_stop && !green);
        self.waiting = at_stop && !green;

        if should_stop {
            return;
        }

        if at_stop && green {
            self.state = VehicleState::InIntersection;
            self.setup_turn();
            return;
        }

        self.move_forward();
    }

    fn setup_turn(&mut self) {
        self.turn_start_x = self.x;
        self.turn_start_y = self.y;
        self.turn_progress = 0.0;

        let (exit_dir, ex, ey) = exit_info(&self.direction, &self.route);
        self.exit_dir = exit_dir;
        self.turn_end_x = ex;
        self.turn_end_y = ey;
    }

    fn update_in_intersection(&mut self) {
        self.turn_progress += VEHICLE_SPEED / 80.0;
        if self.turn_progress >= 1.0 {
            self.turn_progress = 1.0;
            self.x = self.turn_end_x;
            self.y = self.turn_end_y;
            self.state = VehicleState::Exiting;
            return;
        }

        let (cx, cy) = (CENTER_X, CENTER_Y);
        let t = self.turn_progress;
        let mt = 1.0 - t;

        self.x = mt * mt * self.turn_start_x + 2.0 * mt * t * cx + t * t * self.turn_end_x;
        self.y = mt * mt * self.turn_start_y + 2.0 * mt * t * cy + t * t * self.turn_end_y;
    }

    fn update_exiting(&mut self) {
        match self.exit_dir {
            Direction::North => self.y -= VEHICLE_SPEED,
            Direction::South => self.y += VEHICLE_SPEED,
            Direction::East  => self.x += VEHICLE_SPEED,
            Direction::West  => self.x -= VEHICLE_SPEED,
        }
        if self.x < -50.0 || self.x > WINDOW_W + 50.0 || self.y < -50.0 || self.y > WINDOW_H + 50.0 {
            self.state = VehicleState::Done;
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.y -= VEHICLE_SPEED,
            Direction::South => self.y += VEHICLE_SPEED,
            Direction::East  => self.x += VEHICLE_SPEED,
            Direction::West  => self.x -= VEHICLE_SPEED,
        }
    }

    pub fn width(&self) -> f32 {
        match self.direction {
            Direction::North | Direction::South => VEHICLE_W,
            Direction::East | Direction::West => VEHICLE_H,
        }
    }

    pub fn height(&self) -> f32 {
        match self.direction {
            Direction::North | Direction::South => VEHICLE_H,
            Direction::East | Direction::West => VEHICLE_W,
        }
    }
}

fn spawn_pos(dir: &Direction) -> (f32, f32) {
    match dir {
        Direction::North => (CENTER_X + LANE_OFFSET, WINDOW_H + 20.0),
        Direction::South => (CENTER_X - LANE_OFFSET, -20.0),            
        Direction::East  => (-20.0, CENTER_Y + LANE_OFFSET),            
        Direction::West  => (WINDOW_W + 20.0, CENTER_Y - LANE_OFFSET),  
    }
}

fn stop_line_pos(dir: &Direction) -> f32 {
    match dir {
        Direction::North => CENTER_Y + ROAD_HALF, 
        Direction::South => CENTER_Y - ROAD_HALF,  
        Direction::East  => CENTER_X - ROAD_HALF,  
        Direction::West  => CENTER_X + ROAD_HALF, 
    }
}

fn exit_info(dir: &Direction, route: &Route) -> (Direction, f32, f32) {
    match (dir, route) {
        (Direction::North, Route::Straight)   => (Direction::North, CENTER_X + LANE_OFFSET, CENTER_Y - ROAD_HALF),
        (Direction::North, Route::TurnRight)  => (Direction::East,  CENTER_X + ROAD_HALF, CENTER_Y - LANE_OFFSET),
        (Direction::North, Route::TurnLeft)   => (Direction::West,  CENTER_X - ROAD_HALF, CENTER_Y + LANE_OFFSET),
        (Direction::South, Route::Straight)   => (Direction::South, CENTER_X - LANE_OFFSET, CENTER_Y + ROAD_HALF),
        (Direction::South, Route::TurnRight)  => (Direction::West,  CENTER_X - ROAD_HALF, CENTER_Y + LANE_OFFSET),
        (Direction::South, Route::TurnLeft)   => (Direction::East,  CENTER_X + ROAD_HALF, CENTER_Y - LANE_OFFSET),
        (Direction::East, Route::Straight)    => (Direction::East,  CENTER_X + ROAD_HALF, CENTER_Y + LANE_OFFSET),
        (Direction::East, Route::TurnRight)   => (Direction::South, CENTER_X + LANE_OFFSET, CENTER_Y + ROAD_HALF),
        (Direction::East, Route::TurnLeft)    => (Direction::North, CENTER_X - LANE_OFFSET, CENTER_Y - ROAD_HALF),
        (Direction::West, Route::Straight)    => (Direction::West,  CENTER_X - ROAD_HALF, CENTER_Y - LANE_OFFSET),
        (Direction::West, Route::TurnRight)   => (Direction::North, CENTER_X - LANE_OFFSET, CENTER_Y - ROAD_HALF),
        (Direction::West, Route::TurnLeft)    => (Direction::South, CENTER_X + LANE_OFFSET, CENTER_Y + ROAD_HALF),
    }
}
