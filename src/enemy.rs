use crate::{assets::GameAssets, prelude::*, pause_game};

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    gravity: Gravity,
    initial_enemy_speed: InitialEnemySpeed,
    wall_hanging_timer: WallHangingTimer,
    hitbox: HitBox,
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
        game_assets: Res<GameAssets>,
    ) -> Result<Self, String> {
        if (trajectory.x, trajectory.y) < (0.0, 0.0) || (trajectory.x, trajectory.y) > (1.0, 1.0) {
            return Err("The trajectory must be between 0 and 1".to_string());
        }

        let direction = match random::<bool>() {
            true => -1.0,
            false => 1.0,
        };

        let mut starting_x = RIGHT_WALL;

        if direction > 0.0 {
            starting_x = LEFT_WALL;
        }

        Ok(EnemyBundle {
            enemy: Enemy(EnemyState::Airborne),
            velocity: Velocity(Vec2::new(
                trajectory.x * enemy_speed * direction,
                trajectory.y * enemy_speed,
            )),
            gravity: Gravity(gravity),
            initial_enemy_speed: InitialEnemySpeed(enemy_speed * trajectory.y),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: game_assets.red_ninja.clone(),
                transform: Transform {
                    translation: Vec3::new(starting_x, -275.0, 1.0),
                    ..default()
                },
                ..default()
            },
            wall_hanging_timer: WallHangingTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            hitbox: HitBox(Vec2::new(35.0, 60.0)),
        })
    }

    pub fn pawn(game_assets: Res<GameAssets>) -> Self {
        Self::new(1.75, 300.0, Vec2::new(1.0, 1.0), game_assets).unwrap()
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)      
                .with_run_criteria(pause_game)
                .with_system(enemy_spawner)
                .with_system(enemy_movement)
                .with_system(enemy_animator)
                .with_system(gravity_system),
        );
    }
}

pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &InitialEnemySpeed,
            &mut WallHangingTimer,
            &mut Enemy,
        ),
        With<Enemy>,
    >,
    mut commands: Commands,
) {
    for (
        entity,
        mut transform,
        mut velocity,
        initial_enemy_speed,
        mut wall_hanging_timer,
        mut enemy,
    ) in query.iter_mut()
    {
        let Bounds { right, left, .. } = calculate_bounds(&transform, None);

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

pub fn spawn_enemy(commands: &mut Commands, game_assets: Res<GameAssets>) {
    commands.spawn(EnemyBundle::pawn(game_assets));
}

pub fn enemy_spawner(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut count: ResMut<EnemyCount>,
    game_assets: Res<GameAssets>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_enemy(&mut commands, game_assets);
        count.0 += 1;
    }
}

pub fn gravity_system(mut query: Query<(&mut Velocity, &mut Gravity), With<Enemy>>) {
    for (mut velocity, gravity) in query.iter_mut() {
        velocity.y -= gravity.0;
    }
}
