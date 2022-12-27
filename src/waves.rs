use std::collections::HashMap;

use crate::{pause_game, prelude::*};
use ron::de::from_bytes;
use serde::Deserialize;

#[derive(Resource)]
pub struct Level(pub Vec<Wave>, pub usize);

struct LevelDifficultyMap {
    waves: HashMap<WaveDifficulty, Vec<WaveData>>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
enum WaveDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Deserialize, Clone)]
struct WaveData {
    pub enemy_type: EnemyType,
    pub enemy_count: u32,
    pub enemy_interval_sec: f32,
    pub enemy_spawn_timer_sec: f32,
    pub starting_wall: StartingWall,
}

#[derive(Clone, Debug)]
pub struct Wave {
    pub enemy_type: EnemyType,
    pub enemy_count: WaveCount,
    pub enemy_interval: SpawnInterval,
    pub enemy_spawn_timer_sec: EnemySpawnTimer,
    pub starting_wall: StartingWall,
}

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct WaveCount(pub u32);

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum EnemyType {
    Pawn,
}

#[derive(Clone, Debug)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Clone, Debug)]
pub struct SpawnInterval(pub Timer);

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
}

pub struct WavePlugin;
impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        let level_difficulty_map = LevelDifficultyMap {
            waves: from_bytes(include_bytes!("../data/waves.ron")).unwrap(),
        };

        let easy_wave_data = &level_difficulty_map.waves[&WaveDifficulty::Easy];
        let waves = easy_wave_data
            .iter()
            .map(|wave| {
                return Wave {
                    enemy_type: wave.enemy_type.clone(),
                    enemy_count: WaveCount(wave.enemy_count),
                    enemy_interval: SpawnInterval(Timer::from_seconds(
                        wave.enemy_interval_sec,
                        TimerMode::Repeating,
                    )),
                    enemy_spawn_timer_sec: EnemySpawnTimer(Timer::from_seconds(
                        wave.enemy_spawn_timer_sec,
                        TimerMode::Once,
                    )),
                    starting_wall: wave.starting_wall.clone(),
                };
            })
            .collect::<Vec<Wave>>();

        app.insert_resource(Level(waves, 0)).add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(pause_game)
                .with_system(wave_spawner),
        );
    }
}

pub fn wave_spawner(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut level_resource: ResMut<Level>,
    time: Res<Time>,
) {
    let index = level_resource.1;
    let waves = &mut level_resource.0;
    let wave = waves.get_mut(index);

    if let Some(current_wave) = wave {
        if current_wave
            .enemy_spawn_timer_sec
            .0
            .tick(time.delta())
            .finished()
        {
            if current_wave
                .enemy_interval
                .0
                .tick(time.delta())
                .just_finished()
                && current_wave.enemy_count.0 > 0
            {
                commands.spawn(
                    EnemyBundle::new(
                        current_wave.enemy_type,
                        game_assets,
                        current_wave.starting_wall,
                    )
                    .unwrap(),
                );
                current_wave.enemy_count.0 = current_wave.enemy_count.0 - 1;
            } else if current_wave.enemy_interval.0.just_finished() {
                level_resource.1 = level_resource.1 + 1;
            }
        }
    }
}
