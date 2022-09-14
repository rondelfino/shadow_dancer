use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Dimensions(pub Vec2);

#[derive(Component)]
pub struct HitBox(pub Vec2);

#[derive(Component)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct InitialEnemySpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

