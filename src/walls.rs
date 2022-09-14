use bevy::prelude::*;

use crate::{
    components::Wall,
    constants::{LEFT_WALL, RIGHT_WALL},
    Dimensions,
};

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    dimensions: Dimensions,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl WallBundle {
    pub fn left_wall(texture_atlas_handle: Handle<TextureAtlas>, y_pos: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));

        let mut sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(LEFT_WALL - dimensions.0.x / 2.0, y_pos, 0.0),
                ..default()
            },
            ..default()
        };

        sprite_bundle.sprite.index = 0;

        WallBundle {
            wall: Wall,
            dimensions,
            sprite_bundle,
        }
    }

    pub fn right_wall(texture_atlas_handle: Handle<TextureAtlas>, y_pos: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));
        
        let mut sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(RIGHT_WALL + dimensions.0.x / 2.0, y_pos, 0.0),
                ..default()
            },
            ..default()
        };

        sprite_bundle.sprite.index = 1;

        WallBundle {
            wall: Wall,
            dimensions,
            sprite_bundle,
        }
    }
}

pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("objects/walls.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 224.0), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //spawns walls that span offscreen
    for i in -2..=2 {
        commands.spawn().insert_bundle(WallBundle::left_wall(
            texture_atlas_handle.clone(),
            i as f32 * 224.0,
        ));
        commands.spawn().insert_bundle(WallBundle::right_wall(
            texture_atlas_handle.clone(),
            i as f32 * 224.0,
        ));
    }
}

pub fn wall_animator(mut query: Query<(&mut Transform, &Dimensions), With<Wall>>, time: Res<Time>) {
    let wall_speed = 800.0;
    for (mut wall_transform, wall_dimensions) in query.iter_mut() {
        wall_transform.translation.y += wall_speed * time.delta_seconds();

        //repeat wall motion by moving offscreen top wall to the bottom
        if wall_transform.translation.y > 3.0 * wall_dimensions.0.y {
            wall_transform.translation.y = -2.0 * wall_dimensions.0.y
        }
    }
}
