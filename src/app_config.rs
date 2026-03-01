pub const MIN_GREEN_TIME: f32 = 0.5;
pub const CAR_SPEED: f32 = 2.5;

pub const CAR_SIZE: f32 = 24.0;
pub const VEHICLE_LENGTH: f32 = CAR_SIZE;
pub const SAFETY_GAP: f32 = 30.0;
pub const LANE_LENGTH: f32 = 400.0;
pub const LANE_CAPACITY: usize = (LANE_LENGTH / (VEHICLE_LENGTH + SAFETY_GAP)) as usize;

pub const ROAD_WIDTH: f32 = 140.0;
pub const LANE_OFFSET: f32 = 45.0;

pub const CENTER_HALF: f32 = 60.0;
pub const DESPAWN_OFFSET: f32 = 120.0;
pub const SPAWN_DISTANCE_CHECK: f32 = 40.0;
pub const AHEAD_CHECK_DISTANCE: f32 = 50.0;
pub const AHEAD_CHECK_LATERAL: f32 = 18.0;

pub const LIGHT_OFFSET: f32 = 85.0;
pub const LIGHT_SIZE: f32 = 36.0;
pub const LIGHT_PADDING: f32 = 6.0;
