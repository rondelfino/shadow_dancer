use bevy::prelude::*;
use rand::random;

use crate::components::{Enemy, Gravity, ReboundForce, Velocity};

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    gravity: Gravity,
    rebound_force: ReboundForce,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl EnemyBundle {
    /// Creates a new enemy
    ///
    /// # Arguments
    ///
    /// * `world_width` - The horizontal size of the world in pixels
    /// * `gravity` - Downward force acting on the spawned enemy
    /// * `enemy_speed` - Starting speed of the enemy
    /// * `rebound_force` - The force used to calculate the speed of an enemy when changing direction
    /// * `trajectory` - Starting trajectory of the enemy used to calculate launch angle of the enemy; x and y values normalized between 0 and 1
    ///
    pub fn new(
        world_width: f32,
        gravity: f32,
        enemy_speed: f32,
        rebound_force: f32,
        trajectory: Vec2,
    ) -> Result<Self, String> {
        if (trajectory.x, trajectory.y) < (0.0, 0.0) || (trajectory.x, trajectory.y) > (1.0, 1.0) {
            return Err("The trajectory must be between 0 and 1".to_string());
        }

        let direction = match random::<bool>() {
            true => -1.,
            false => 1.,
        };

        Ok(EnemyBundle {
            enemy: Enemy,
            velocity: Velocity(trajectory * enemy_speed),
            gravity: Gravity(gravity),
            rebound_force: ReboundForce(rebound_force),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(direction * world_width / 2.0, -350.0, 0.0),
                    ..default()
                },
                ..default()
            },
        })
    }

    pub fn pawn(world_width: f32) -> Self {
        Self::new(world_width, 4.5, 1000.0, 700.0, Vec2::new(1.0, 0.25)).unwrap()
    }
}
