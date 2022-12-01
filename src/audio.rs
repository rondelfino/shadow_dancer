use bevy::prelude::*;
use bevy_kira_audio::prelude::{
    Audio as KiraAudio, AudioApp, AudioChannel, AudioControl, AudioEasing,
    AudioPlugin as KiraAudioPlugin, AudioSource as KiraAudioSource, AudioTween,
};

use std::time::Duration;

use crate::{collision::collision_system, GameSystemLabel};

pub struct GameAudioPlugin;

#[derive(Resource)]
pub struct BGMChannel;

#[derive(Resource)]
pub struct SFXChannel;

#[derive(Resource)]
pub struct SFXHandles {
    collision_sound: Handle<KiraAudioSource>,
    death_sound: Handle<KiraAudioSource>,
    shuriken_sound: Handle<KiraAudioSource>,
}

pub enum SFXEvents {
    CollisionSound,
    DeathSound,
    ShurikenSound,
}

#[derive(Resource)]
pub struct BGMHandles {
    bgm_01: Handle<KiraAudioSource>,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraAudioPlugin)
            .add_audio_channel::<BGMChannel>()
            .add_audio_channel::<SFXChannel>()
            .add_startup_system(load_audio.label(GameSystemLabel::Core))
            .add_startup_system(set_audio_channel_volume.label(GameSystemLabel::Core))
            .add_startup_system(play_bgm.label(GameSystemLabel::Core))
            .add_system_set(
                SystemSet::new()
                    .with_system(collision_system)
                    .with_system(play_sfx.after(collision_system))
                    .label(GameSystemLabel::Core),
            );
    }
}

pub fn set_audio_channel_volume(
    music_channel: Res<AudioChannel<BGMChannel>>,
    effects_channel: Res<AudioChannel<SFXChannel>>,
) {
    music_channel.set_volume(0.025);
    effects_channel.set_volume(0.075);
}

pub fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let collision_sound = asset_server.load("enemy/impact.ogg");
    let death_sound = asset_server.load("effects/disintegrate.ogg");
    let shuriken_sound = asset_server.load("player/shuriken.ogg");

    let bgm_01 = asset_server.load("music/17-Union-Lizard-_Final-Boss_-Keisuke-Tsukahara.ogg");

    commands.insert_resource(SFXHandles {
        collision_sound,
        death_sound,
        shuriken_sound,
    });

    commands.insert_resource(BGMHandles { bgm_01 });
}

fn play_sfx(
    audio: Res<AudioChannel<SFXChannel>>,
    sound: Res<SFXHandles>,
    mut sfx_events: EventReader<SFXEvents>,
) {
    for event in sfx_events.iter() {
        match event {
            SFXEvents::CollisionSound => {
                audio.play(sound.collision_sound.clone());
            }
            SFXEvents::DeathSound => {
                audio.play(sound.death_sound.clone());
            }
            SFXEvents::ShurikenSound => {
                audio.play(sound.shuriken_sound.clone());
            }
        }
    }

    if !sfx_events.is_empty() {
        sfx_events.clear();
    }
}

pub fn play_bgm(audio: Res<AudioChannel<BGMChannel>>, asset_server: Res<AssetServer>) {
    audio
        .play(asset_server.load("music/15 - Statue of Liberty (Round 3-2) - Keisuke Tsukahara.ogg"))
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        .looped();
}
