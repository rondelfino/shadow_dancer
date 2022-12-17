#![allow(clippy::type_complexity)]
mod prelude {
    pub use crate::audio::*;
    pub use crate::background::*;
    pub use crate::camera::*;
    pub use crate::collision::*;
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::death_effect::*;
    pub use crate::enemy::*;
    pub use crate::player::*;
    pub use crate::resources::*;
    pub use crate::roof::*;
    pub use crate::shuriken::*;
    pub use crate::utils::*;
    pub use crate::walls::*;
    pub use bevy::{
        math::Vec3Swizzles, prelude::*, render::camera::ScalingMode, sprite::collide_aabb::collide,
    };
    pub use bevy_kira_audio::prelude::{
        AudioApp, AudioChannel, AudioControl, AudioEasing, AudioPlugin as KiraAudioPlugin,
        AudioSource as KiraAudioSource, AudioTween,
    };
    pub use rand::random;
    pub use std::time::Duration;
}

use asset_loading::AssetsLoadingPlugin;
use assets::AssetsPlugin;
use game_script::GameScriptPlugin;

use crate::prelude::*;

mod asset_loading;
mod assets;
mod audio;
mod background;
mod camera;
mod collision;
mod components;
mod constants;
mod death_effect;
mod enemy;
mod game_script;
mod player;
mod resources;
mod roof;
mod shuriken;
mod utils;
mod walls;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
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
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(AssetsLoadingPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GameScriptPlugin)
        .add_plugin(RoofPlugin)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(EnemyCount(0))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_system(enemy_spawner)
                .with_system(shuriken_movement)
                .with_system(shuriken_animator)
                .with_system(enemy_animator)
                .with_system(enemy_movement)
                .with_system(gravity_system)
                .with_system(death_effect_animator)
                .with_system(wall_animator),
        )
        .add_system(
            despawner
                .after(GameSystemLabel::Core)
                .label(GameSystemLabel::Cleanup),
        )
        .add_state(GameState::Initial)
        .add_system_set(SystemSet::on_update(GameState::Initial).with_system(bootstrap))
        .run();
}

fn bootstrap(
    mut assets_handler: asset_loading::AssetHandler,
    mut game_assets: ResMut<assets::GameAssets>,
) {
    assets_handler.load(GameState::LoadWorld, &mut game_assets);
}

fn test(game_state: Res<State<GameState>>) {
    println!("{:?}", game_state);
}
