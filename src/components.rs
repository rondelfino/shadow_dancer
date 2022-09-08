use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct ReboundForce(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
