use crate::{assets::GameAssets, pause_game, prelude::*};
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Clone, Debug)]
pub struct SpawnInterval(pub Timer);

#[derive(Component, Debug)]
pub struct ReflectChance(pub f32);

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    gravity: Gravity,
    initial_enemy_speed: InitialEnemySpeed,
    wall_hanging_timer: WallHangingTimer,
    hitbox: HitBox,
    sprite_bundle: SpriteSheetBundle,
    reflect_chance: ReflectChance,
    dimensions: Dimensions,
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
        enemy_type: EnemyType,
        game_assets: Res<GameAssets>,
        starting_wall: StartingWall,
    ) -> Result<Self, String> {
        let trajectory = enemy_type.get_trajectory();
        let enemy_speed = enemy_type.get_speed();
        let reflect_chance = enemy_type.get_reflect_chance();

        if (trajectory.x, trajectory.y) < (0.0, 0.0) || (trajectory.x, trajectory.y) > (1.0, 1.0) {
            return Err("The trajectory must be between 0 and 1".to_string());
        }

        let direction = match starting_wall {
            StartingWall::Left => 1.0,
            StartingWall::Right => -1.0,
        };

        let mut starting_x = RIGHT_WALL;

        if direction > 0.0 {
            starting_x = LEFT_WALL;
        }

        let mut rng = thread_rng();

        Ok(EnemyBundle {
            enemy: Enemy(EnemyState::Airborne),
            velocity: Velocity(Vec2::new(
                trajectory.x * enemy_speed * direction,
                trajectory.y * enemy_speed,
            )),
            gravity: Gravity(enemy_type.get_gravity()),
            initial_enemy_speed: InitialEnemySpeed(enemy_speed * trajectory.y),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: enemy_type.get_texture(game_assets),
                transform: Transform {
                    translation: Vec3::new(starting_x, -275.0, 1.0),
                    ..default()
                },
                ..default()
            },
            wall_hanging_timer: WallHangingTimer(Timer::from_seconds(
                rng.gen_range(0.1..0.4),
                TimerMode::Repeating,
            )),
            hitbox: HitBox(Vec2::new(35.0, 60.0)),
            reflect_chance: ReflectChance(reflect_chance),
            dimensions: Dimensions(Vec2::new(30.0, 30.0)),
        })
    }
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum EnemyType {
    Pawn,
}

impl EnemyType {
    pub fn get_texture(&self, game_assets: Res<GameAssets>) -> Handle<TextureAtlas> {
        match self {
            EnemyType::Pawn => game_assets.red_ninja.clone(),
        }
    }

    pub fn get_gravity(&self) -> f32 {
        match self {
            EnemyType::Pawn => 7.0,
        }
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            EnemyType::Pawn => 600.0,
        }
    }

    pub fn get_trajectory(&self) -> Vec2 {
        match self {
            EnemyType::Pawn => Vec2::new(1.0, 1.0),
        }
    }

    pub fn get_reflect_chance(&self) -> f32 {
        match self {
            EnemyType::Pawn => 0.20,
        }
    }
}

pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &InitialEnemySpeed,
        &mut WallHangingTimer,
        &mut Enemy,
        &Dimensions,
    )>,
    mut commands: Commands,
) {
    for (
        entity,
        mut transform,
        mut velocity,
        initial_enemy_speed,
        mut wall_hanging_timer,
        mut enemy,
        dimensions,
    ) in query.iter_mut()
    {
        let Bounds { right, left, .. } = calculate_bounds(&transform, Some(dimensions.0));

        let is_touching_left_bound = left < LEFT_WALL;
        let is_touching_right_bound = right > RIGHT_WALL;

        if (velocity.x < 0.0 && is_touching_left_bound)
            || (velocity.x > 0.0 && is_touching_right_bound)
        {
            if wall_hanging_timer.0.tick(time.delta()).just_finished() {
                velocity.x *= -1.0;
                velocity.y = initial_enemy_speed.0;
                enemy.0 = EnemyState::Airborne;
            } else {
                enemy.0 = EnemyState::WallHanging;
            }
        } else {
            transform.translation.y += velocity.y * time.delta().as_secs_f32();
            transform.translation.x += velocity.x * time.delta().as_secs_f32();
        }

        if transform.translation.y > (WORLD_HEIGHT / 2.0) + 100.0 && enemy.0 != EnemyState::Dead {
            commands.entity(entity).insert(MarkDespawn);
        }
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

pub fn gravity_system(mut query: Query<(&mut Velocity, &mut Gravity), With<Enemy>>) {
    for (mut velocity, gravity) in query.iter_mut() {
        velocity.y -= gravity.0;
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(pause_game)
                .with_system(enemy_movement)
                .with_system(enemy_animator)
                .with_system(gravity_system),
        );
    }
}
