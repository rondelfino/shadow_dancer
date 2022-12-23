use crate::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub enum EnemyState {
    Airborne,
    WallHanging,
    Dead,
}

#[derive(PartialEq, Eq, Debug)]
pub enum PlayerAction {
    Idle,
    WalkingLeft,
    WalkingRight,
    Flipping,
    Falling,
    Landing,
    Landed,
    Attacking,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LevelState {
    Intro,
    Start,
}

#[derive(Component)]
pub struct AttackCooldown(pub Timer);

#[derive(Component)]
pub struct MarkDespawn;

#[derive(Component)]
pub struct Player(pub PlayerAction, pub LevelState);

#[derive(Component)]
pub struct Enemy(pub EnemyState);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Roof;

#[derive(Component)]
pub struct Shuriken;

#[derive(Component)]
pub struct Background;

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

#[derive(Component, Deref, DerefMut)]
pub struct WalkingAnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct FlippingAnimationTimer(pub Timer);

#[derive(Component)]
pub struct Effect;


