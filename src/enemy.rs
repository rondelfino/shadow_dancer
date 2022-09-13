use bevy::prelude::*;
use rand::random;

use crate::components::{Enemy, Gravity, InitialEnemySpeed, Velocity};

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    gravity: Gravity,
    initial_enemy_speed: InitialEnemySpeed,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl EnemyBundle {
    /// Creates a new enemy
    ///
    /// # Arguments
    ///
    /// * `world_width` - The horizontal size of the world in pixels
    /// * `gravity` - Downward force acting on the spawned enemy
    /// * `enemy_speed` - Starting speed of the enemy
    /// * `initial_enemy_speed` - The force used to calculate the speed of an enemy when changing direction
    /// * `trajectory` - Starting trajectory of the enemy used to calculate launch angle of the enemy; x and y values normalized between 0 and 1
    ///
    pub fn new(
        world_width: f32,
        gravity: f32,
        enemy_speed: f32,
        trajectory: Vec2,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Result<Self, String> {
        if (trajectory.x, trajectory.y) < (0.0, 0.0) || (trajectory.x, trajectory.y) > (1.0, 1.0) {
            return Err("The trajectory must be between 0 and 1".to_string());
        }

        let direction = match random::<bool>() {
            true => -1.,
            false => 1.,
        };

        let texture_handle = asset_server.load("enemy/red_ninja.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 65.0), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Ok(EnemyBundle {
            enemy: Enemy,
            velocity: Velocity(trajectory * enemy_speed),
            gravity: Gravity(gravity),
            initial_enemy_speed: InitialEnemySpeed(enemy_speed * trajectory.y),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(direction * world_width / 2.0, -350.0, 0.0),
                    ..default()
                },
                ..default()
            },
        })
    }

    pub fn pawn(
        world_width: f32,
        asset_server: Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self::new(
            world_width,
            1.75,
            300.0,
            Vec2::new(1.0, 1.0),
            asset_server,
            texture_atlases,
        )
        .unwrap()
    }
}
