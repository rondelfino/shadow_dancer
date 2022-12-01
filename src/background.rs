use bevy::prelude::*;

use crate::{
    components::{Background, Dimensions},
    constants::FALLING_SPEED,
};

#[derive(Bundle)]
pub struct BackgroundBundle {
    background: Background,
    dimensions: Dimensions,
    sprite_bundle: SpriteBundle,
}

impl BackgroundBundle {
    pub fn new(texture_image_handle: Handle<Image>) -> Self {
        let dimensions = Dimensions(Vec2::new(1248.0, 1667.0));

        let sprite_bundle = SpriteBundle {
            texture: texture_image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, -dimensions.0.y / 7.5, 0.0),
                scale: Vec3::new(0.63, 0.5, 0.0),
                ..default()
            },
            ..default()
        };
        BackgroundBundle {
            background: Background,
            dimensions,
            sprite_bundle,
        }
    }
}

// pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let texture_image_handle = asset_server.load("background/skybox.png");

//     commands.spawn(SpriteBundle {
//         texture: texture_image_handle,
//         transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.0)),

//         ..default()
//     });
// }

pub fn spawn_day_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_image_handle: Handle<Image> = asset_server.load("background/skybox.png");

    commands
        .spawn_empty()
        .insert(BackgroundBundle::new(texture_image_handle));
}

pub fn background_animator(mut query: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    for mut background_transform in query.iter_mut() {
        background_transform.translation.y += (FALLING_SPEED / 100.0) * time.delta_seconds();
    }
}
