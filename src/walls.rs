use crate::prelude::*;

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    dimensions: Dimensions,
    sprite_bundle: SpriteSheetBundle,
}

impl WallBundle {
    pub fn left_wall(
        texture_atlas_handle: Handle<TextureAtlas>,
        index: i32,
        left_roof_height: f32,
    ) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 244.0));

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(
                    LEFT_WALL - dimensions.0.x / 2.0,
                    (index as f32 * dimensions.0.y)
                        - (left_roof_height / 2.0 + dimensions.0.y / 2.0)
                        - 100.0,
                    0.0,
                ),
                ..default()
            },
            ..default()
        };

        WallBundle {
            wall: Wall,
            dimensions,
            sprite_bundle,
        }
    }

    pub fn right_wall(
        texture_atlas_handle: Handle<TextureAtlas>,
        index: i32,
        right_roof_height: f32,
    ) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 244.0));

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(
                    RIGHT_WALL + dimensions.0.x / 2.0,
                    (index as f32 * dimensions.0.y)
                        - (right_roof_height / 2.0 + dimensions.0.y / 2.0)
                        - 100.0,
                    0.0,
                ),
                ..default()
            },
            ..default()
        };

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
    left_roof_height: f32,
    right_roof_height: f32,
) {
    let left_texture_handle = asset_server.load("background/left_wall.png");
    let left_texture_atlas = TextureAtlas::from_grid(
        left_texture_handle,
        Vec2::new(48.0, 244.0),
        2,
        1,
        None,
        None,
    );
    let left_texture_atlas_handle = texture_atlases.add(left_texture_atlas);

    let right_texture_handle = asset_server.load("background/right_wall.png");
    let right_texture_atlas = TextureAtlas::from_grid(
        right_texture_handle,
        Vec2::new(48.0, 244.0),
        2,
        1,
        None,
        None,
    );
    let right_texture_atlas_handle = texture_atlases.add(right_texture_atlas);

    //spawns walls that span offscreen
    for i in -4..=0 {
        commands.spawn_empty().insert(WallBundle::left_wall(
            left_texture_atlas_handle.clone(),
            i,
            left_roof_height,
        ));
        commands.spawn_empty().insert(WallBundle::right_wall(
            right_texture_atlas_handle.clone(),
            i,
            right_roof_height,
        ));
    }
}

pub fn wall_animator(mut query: Query<(&mut Transform, &Dimensions), With<Wall>>, time: Res<Time>) {
    for (mut wall_transform, wall_dimensions) in query.iter_mut() {
        wall_transform.translation.y += FALLING_SPEED * time.delta_seconds();

        //repeat wall motion by moving offscreen top wall to the bottom
        if wall_transform.translation.y > 1.5 * wall_dimensions.0.y {
            wall_transform.translation.y -= 3.0 * wall_dimensions.0.y
        }
    }
}
