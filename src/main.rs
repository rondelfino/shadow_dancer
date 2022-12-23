#![allow(clippy::type_complexity)]
mod prelude {
    pub use crate::asset_loading::*;
    pub use crate::assets::*;
    pub use crate::audio::*;
    pub use crate::background::*;
    pub use crate::camera::*;
    pub use crate::collision::*;
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::death_effect::*;
    pub use crate::enemy::*;
    pub use crate::game_script::*;
    pub use crate::pause_menu::*;
    pub use crate::player::*;
    pub use crate::resources::*;
    pub use crate::roof::*;
    pub use crate::shuriken::*;
    pub use crate::utils::*;
    pub use crate::walls::*;

    pub use bevy::{
        math::Vec3Swizzles, prelude::*, render::camera::ScalingMode, sprite::collide_aabb::collide,
    };
    pub use bevy_easings::*;
    pub use bevy_kira_audio::prelude::{
        AudioApp, AudioChannel, AudioControl, AudioEasing, AudioPlugin as KiraAudioPlugin,
        AudioSource as KiraAudioSource, AudioTween,
    };

    pub use bevy::ecs::schedule::ShouldRun;
    pub use rand::random;
    pub use std::time::Duration;
}

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
mod pause_menu;
mod player;
mod resources;
mod roof;
mod shuriken;
mod utils;
mod walls;

pub fn pause_game(event: Res<PauseEvent>, query: Query<&Player>) -> ShouldRun {
    let player = query.get_single();
    match player {
        Ok(p) => {
            if *event == PauseEvent::Unpaused && p.1 == LevelState::Start {
                ShouldRun::Yes
            } else {
                ShouldRun::No
            }
        }
        Err(_) => ShouldRun::No,
    }
}

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
        .init_resource::<PauseEvent>()
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(AssetsLoadingPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GameScriptPlugin)
        .add_plugin(RoofPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(ShurikenPlugin)
        .add_plugin(PauseMenuPlugin)
        .add_plugin(EasingsPlugin)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(EnemyCount(0))
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(death_effect_animator))
        .add_system_to_stage(CoreStage::PostUpdate, despawner::<MarkDespawn>)
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
