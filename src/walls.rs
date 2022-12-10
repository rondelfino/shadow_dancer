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
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));

        let mut sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(
                    LEFT_WALL - dimensions.0.x,
                    (index as f32 * dimensions.0.y)
                        - (left_roof_height / 2.0 + dimensions.0.y / 2.0)
                        - 100.0,
                    0.0,
                ),
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

    pub fn right_wall(
        texture_atlas_handle: Handle<TextureAtlas>,
        index: i32,
        right_roof_height: f32,
    ) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));

        let mut sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(
                    RIGHT_WALL + dimensions.0.x,
                    (index as f32 * dimensions.0.y)
                        - (right_roof_height / 2.0 + dimensions.0.y / 2.0)
                        - 100.0,
                    0.0,
                ),
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
    left_roof_height: f32,
    right_roof_height: f32,
) {
    let texture_handle = asset_server.load("objects/walls.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 224.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //spawns walls that span offscreen
    for i in -4..=0 {
        commands.spawn_empty().insert(WallBundle::left_wall(
            texture_atlas_handle.clone(),
            i,
            left_roof_height,
        ));
        commands.spawn_empty().insert(WallBundle::right_wall(
            texture_atlas_handle.clone(),
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
