#![allow(clippy::type_complexity)]
use audio::{GameAudioPlugin, SFXEvents};
use background::*;
use bevy::prelude::*;
use camera::camera_setup;
use collision::collision_system;
use components::*;
use death_effect::death_effect_animator;
use enemy::*;
use player::*;
use resources::*;
use roof::{build_towers, roof_animator};
use shuriken::{shuriken_animator, shuriken_movement};
use utils::*;
use walls::wall_animator;

mod audio;
mod background;
mod camera;
mod collision;
mod components;
mod constants;
mod death_effect;
mod enemy;
mod player;
mod resources;
mod roof;
mod shuriken;
mod utils;
mod walls;
// mod settings;
// mod systems;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Shadow Dancer".to_string(),
                        mode: WindowMode::Windowed,
                        scale_factor_override: Some(1.0),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(PlayerPlugin)
        .add_plugin(GameAudioPlugin)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(EnemyCount(0))
        .add_startup_system(camera_setup)
        .add_event::<SFXEvents>()
        .add_state(GameState::StageIntro)
        .add_startup_system(build_towers)
        .add_startup_system(spawn_background)
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_system(collision_system)
                .with_system(background_animator)
                .with_system(roof_animator)
                .with_system(enemy_spawner)
                .with_system(shuriken_movement)
                .with_system(shuriken_animator)
                .with_system(enemy_animator)
                .with_system(enemy_movement)
                .with_system(gravity_system)
                .with_system(death_effect_animator)
                .with_system(collision_system)
                .with_system(wall_animator),
        )
        .add_system(
            despawner
                .after(GameSystemLabel::Core)
                .label(GameSystemLabel::Cleanup),
        )
        .run();
}
