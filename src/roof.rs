use bevy::prelude::*;

use crate::{
    components::{Dimensions, MarkDespawn, Roof},
    constants::{BOTTOM_WALL, FALLING_SPEED, LEFT_WALL, RIGHT_WALL},
};

#[derive(Bundle)]
pub struct RoofBundle {
    roof: Roof,
    dimensions: Dimensions,
    sprite_bundle: SpriteSheetBundle,
}

impl RoofBundle {
    pub fn left_wall_roof(texture_atlas_handle: Handle<TextureAtlas>, y_pos: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(174.0, 120.0));

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(LEFT_WALL + dimensions.0.x / 4.5, y_pos, 0.0),
                ..default()
            },
            ..default()
        };
        RoofBundle {
            roof: Roof,
            dimensions,
            sprite_bundle,
        }
    }

    pub fn right_wall_roof(texture_atlas_handle: Handle<TextureAtlas>, y_pos: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 103.0));

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(RIGHT_WALL + dimensions.0.x / 2.0, y_pos, 0.0),
                ..default()
            },
            ..default()
        };

        RoofBundle {
            roof: Roof,
            dimensions: dimensions,
            sprite_bundle: sprite_bundle,
        }
    }
}

pub fn spawn_roofs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle_left = asset_server.load("objects/left_roof.png");
    let texture_atlas_left = TextureAtlas::from_grid(
        texture_handle_left,
        Vec2::new(174.0, 224.0),
        1,
        1,
        None,
        None,
    );
    let texture_atlas_handle_left = texture_atlases.add(texture_atlas_left);

    let texture_handle_right = asset_server.load("objects/right_roof.png");
    let texture_atlas_right = TextureAtlas::from_grid(
        texture_handle_right,
        Vec2::new(48.0, 224.0),
        1,
        1,
        None,
        None,
    );
    let texture_atlas_handle_right = texture_atlases.add(texture_atlas_right);

    commands.spawn_empty().insert(RoofBundle::left_wall_roof(
        texture_atlas_handle_left.clone(),
        BOTTOM_WALL + 39.51,
    ));
    commands.spawn_empty().insert(RoofBundle::right_wall_roof(
        texture_atlas_handle_right.clone(),
        BOTTOM_WALL + 31.1,
    ));
}

pub fn roof_animator(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Dimensions), With<Roof>>,
    time: Res<Time>,
) {
    for (roofs, mut roof_transform, roof_dimensions) in query.iter_mut() {
        roof_transform.translation.y += FALLING_SPEED * time.delta_seconds();

        if roof_transform.translation.y > 3.0 * roof_dimensions.0.y {
            commands.entity(roofs).insert(MarkDespawn);
        }
    }
}
