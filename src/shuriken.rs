use crate::{assets::GameAssets, pause_game, prelude::*};

#[derive(Component)]
pub struct ShurikenSpeed(pub f32);

#[derive(Component)]
pub struct Reflected(pub f32);

#[derive(Bundle)]
pub struct ShurikenBundle {
    shuriken: Shuriken,
    animation_timer: AnimationTimer,
    velocity: Velocity,
    hitbox: HitBox,
    sprite_bundle: SpriteBundle,
    speed: ShurikenSpeed,
}

impl ShurikenBundle {
    pub fn new(game_assets: Res<GameAssets>, starting_pos: Vec3, shuriken_speed: f32) -> Self {
        ShurikenBundle {
            shuriken: Shuriken,
            animation_timer: AnimationTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            velocity: Velocity(Vec2::new(0.0, -400.0)),
            hitbox: HitBox(Vec2::new(8.0, 8.0)),
            speed: ShurikenSpeed(shuriken_speed),
            sprite_bundle: SpriteBundle {
                texture: game_assets.shuriken.clone(),
                transform: Transform {
                    translation: starting_pos,
                    ..Default::default()
                },
                ..default()
            },
        }
    }
}

pub struct ShurikenPlugin;
impl Plugin for ShurikenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(pause_game)
                .with_system(shuriken_movement)
                .with_system(shuriken_animator)
                .with_system(reflect_projectile),
        );
    }
}

pub fn shuriken_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity, &mut ShurikenSpeed), With<Shuriken>>,
) {
    for (entity, mut transform, velocity, mut speed) in query.iter_mut() {
        let Bounds { right, left, .. } = calculate_bounds(&transform, None);

        let is_touching_left_bound = left < LEFT_WALL;
        let is_touching_right_bound = right > RIGHT_WALL;

        if is_touching_left_bound || is_touching_right_bound {
            speed.0 = 0.0;
            transform.translation.y += FALLING_SPEED * time.delta().as_secs_f32();
        } else {
            transform.translation.y += velocity.y * time.delta().as_secs_f32();
            transform.translation.x += velocity.x * time.delta().as_secs_f32();
            if transform.translation.y < (-WORLD_HEIGHT / 2.0) - 100.0 {
                commands.entity(entity).insert(MarkDespawn);
            }
        }
    }
}

pub fn shuriken_animator(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ShurikenSpeed), With<Shuriken>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        if speed.0 > 0.0 {
            transform.rotate_z(f32::to_radians(1060.0) * time.delta_seconds());
        }
    }
}

pub fn reflect_projectile(
    mut shuriken_query: Query<
        (&mut Velocity, &Reflected, &ShurikenSpeed),
        (With<Shuriken>, With<Reflected>, Without<MarkDespawn>),
    >,
) {
    for (mut shuriken_velocity, reflected, speed) in shuriken_query.iter_mut() {
        shuriken_velocity.x = speed.0 * reflected.0.cos();
        shuriken_velocity.y = speed.0 * reflected.0.sin();
    }
}
