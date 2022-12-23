use crate::prelude::*;

pub const WORLD_WIDTH: f32 = 480.0;
pub const WORLD_HEIGHT: f32 = 640.0;

pub const ASPECT_RATIO: f32 = WORLD_WIDTH / WORLD_HEIGHT;

pub const CAMERA_SCALE: f32 = 0.6;
pub const LEFT_WALL: f32 = -WORLD_WIDTH / 2.0;
pub const RIGHT_WALL: f32 = WORLD_WIDTH / 2.0;
pub const UPPER_BOUND: f32 = (WORLD_HEIGHT / 2.0) * CAMERA_SCALE - 20.5;
pub const LOWER_BOUND: f32 = -(WORLD_HEIGHT / 2.0) * CAMERA_SCALE + 20.5;
pub const BONUS_STAGE_INTRO_RIGHT_BOUNDARY: f32 = -WORLD_WIDTH / 4.5;
pub const BONUS_STAGE_SPAWN_POS: Vec2 = Vec2::new(-WORLD_WIDTH / 3.5, -28.0);

pub const FALLING_SPEED: f32 = 600.0;
