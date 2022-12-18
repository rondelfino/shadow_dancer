use crate::{assets::GameAssets, prelude::*};

#[derive(Bundle)]
pub struct ShurikenBundle {
    shuriken: Shuriken,
    animation_timer: AnimationTimer,
    velocity: Velocity,
    hitbox: HitBox,
    sprite_bundle: SpriteBundle,
}

impl ShurikenBundle {
    pub fn new(game_assets: Res<GameAssets>, starting_pos: Vec3) -> Self {
        ShurikenBundle {
            shuriken: Shuriken,
            animation_timer: AnimationTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            velocity: Velocity(Vec2::new(0.0, -400.0)),
            hitbox: HitBox(Vec2::new(8.0, 8.0)),
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
                .label(GameSystemLabel::Core)
                .with_system(shuriken_movement)
                .with_system(shuriken_animator),
        );
    }
}

pub fn shuriken_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Shuriken>>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.y * time.delta().as_secs_f32();
        transform.translation.x += velocity.x * time.delta().as_secs_f32();
        if transform.translation.y < (-WORLD_HEIGHT / 2.0) - 100.0 {
            commands.entity(entity).insert(MarkDespawn);
        }
    }
}

pub fn shuriken_animator(time: Res<Time>, mut query: Query<&mut Transform, With<Shuriken>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(f32::to_radians(1060.0) * time.delta_seconds());
    }
}
