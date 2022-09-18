use bevy::prelude::*;

#[derive(PartialEq, Eq)]
pub enum EnemyState {
    Airborne,
    WallHanging,
    Dead,
}
#[derive(PartialEq, Eq)]
pub enum PlayerState {
    Falling,
    Attacking,
}

#[derive(Component)]
pub struct Player(pub PlayerState);

#[derive(Component)]
pub struct Enemy(pub EnemyState);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Shuriken;

#[derive(Component)]
pub struct WallHangingTimer(pub Timer);

#[derive(Component)]
pub struct AttackingTimer(pub Timer);

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
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Effect;
