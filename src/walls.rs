use crate::{assets::GameAssets, prelude::*, pause_game};

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    dimensions: Dimensions,
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    pub fn left_wall(game_assets: &Res<GameAssets>, index: i32, left_roof_height: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));

        let sprite_bundle = SpriteBundle {
            texture: game_assets.left_wall.clone(),
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

    pub fn right_wall(game_assets: &Res<GameAssets>, index: i32, right_roof_height: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));

        let sprite_bundle = SpriteBundle {
            texture: game_assets.right_wall.clone(),
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

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(pause_game)
                .with_system(wall_animator),
        );
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
