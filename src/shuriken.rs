use bevy::prelude::*;

use crate::{components::*, constants::WORLD_HEIGHT};

#[derive(Bundle)]
pub struct ShurikenBundle {
    shuriken: Shuriken,
    animation_timer: AnimationTimer,
    velocity: Velocity,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl ShurikenBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        starting_pos: Vec3,
    ) -> Self {
        let texture_handle = asset_server.load("objects/shuriken.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        ShurikenBundle {
            shuriken: Shuriken,
            velocity: Velocity(Vec2::new(0.0, -500.0)),
            animation_timer: AnimationTimer(Timer::from_seconds(2.0, true)),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: starting_pos,
                    ..Default::default()
                },
                ..default()
            },
        }
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
            commands.entity(entity).despawn();
        }
    }
}
