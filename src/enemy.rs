use bevy::prelude::*;
use rand::random;

use crate::{
    components::{Enemy, EnemyState, Gravity, InitialEnemySpeed, Velocity, WallHangingTimer},
    constants::{LEFT_WALL, RIGHT_WALL},
};

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    gravity: Gravity,
    initial_enemy_speed: InitialEnemySpeed,
    wall_hanging_timer: WallHangingTimer,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl EnemyBundle {
    /// Creates a new enemy
    ///
    /// # Arguments
    ///
    /// * `gravity` - Downward force acting on the spawned enemy
    /// * `enemy_speed` - Starting speed of the enemy
    /// * `initial_enemy_speed` - The force used to calculate the speed of an enemy when changing direction
    /// * `trajectory` - Starting trajectory of the enemy used to calculate launch angle of the enemy; x and y values normalized between 0 and 1
    ///
    pub fn new(
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

        let mut starting_x = RIGHT_WALL;

        if direction > 0. {
            starting_x = LEFT_WALL;
        }

        let texture_handle = asset_server.load("enemy/red_ninja.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 65.0), 4, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Ok(EnemyBundle {
            enemy: Enemy(EnemyState::Airborne),
            velocity: Velocity(Vec2::new(
                trajectory.x * enemy_speed * direction,
                trajectory.y * enemy_speed,
            )),
            gravity: Gravity(gravity),
            initial_enemy_speed: InitialEnemySpeed(enemy_speed * trajectory.y),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(starting_x, -275.0, 1.0),
                    ..default()
                },
                ..default()
            },
            wall_hanging_timer: WallHangingTimer(Timer::from_seconds(0.1, true))
        })
    }

    pub fn pawn(
        asset_server: Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self::new(
            1.75,
            300.0,
            Vec2::new(1.0, 1.0),
            asset_server,
            texture_atlases,
        )
        .unwrap()
    }
}

pub fn enemy_animator(mut query: Query<(&Enemy, &Velocity, &mut TextureAtlasSprite), With<Enemy>>) {
    for (enemy, velocity, mut sprite) in query.iter_mut() {
        if enemy.0 == EnemyState::WallHanging {
            if velocity.x > 0.0 {
                sprite.index = 3;
            } else {
                sprite.index = 2;
            }
        } else if enemy.0 == EnemyState::Airborne {
            if velocity.x < 0.0 {
                sprite.index = 1;
            } else {
                sprite.index = 0;
            }
        }
    }
}
