use crate::{assets::GameAssets, prelude::*};

#[derive(Bundle)]
pub struct DeathEffectBundle {
    effect: Effect,
    animation_timer: AnimationTimer,
    velocity: Velocity,
    sprite_bundle: SpriteSheetBundle,
}

impl DeathEffectBundle {
    pub fn new(game_assets: &Res<GameAssets>, starting_pos: Vec3) -> Self {
        DeathEffectBundle {
            effect: Effect,
            animation_timer: AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            velocity: Velocity(Vec2::new(0.0, FALLING_SPEED)),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: game_assets.death_effect.clone(),
                transform: Transform {
                    translation: starting_pos,
                    ..Default::default()
                },
                ..default()
            },
        }
    }
}

pub fn death_effect_animator(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &mut Transform,
            &Velocity,
        ),
        With<Effect>,
    >,
) {
    for (entity, mut sprite, mut animation_timer, mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_seconds();

        if animation_timer.0.tick(time.delta()).just_finished() {
            if sprite.index >= 3 {
                commands.entity(entity).insert(MarkDespawn);
           
            }

            if sprite.index < 3 {
                sprite.index += 1;
            }
        }
    }
}
