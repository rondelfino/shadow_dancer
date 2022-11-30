use bevy::prelude::*;
use bevy_kira_audio::{
    Audio as KiraAudio, AudioChannel, AudioControl, AudioPlugin as KiraAudioPlugin,
    AudioSource as KiraAudioSource,
};

use crate::{collision::collision_system, GameSystemLabel};

pub struct GameAudioPlugin;

#[derive(Resource)]
pub struct MusicChannel;
#[derive(Resource)]
pub struct EffectsChannel;

#[derive(Resource)]
pub struct SFXHandles {
    collision_sound: Handle<KiraAudioSource>,
    death_sound: Handle<KiraAudioSource>,
    shuriken_sound: Handle<KiraAudioSource>,
}

pub enum SFXEvents {
    CollisionSound,
    DeathSound,
    ShurikenSound
}

// #[derive(Resource)]
// pub struct AudioSate {
//     enemy_death_handle: Handle<AudioSource>,
//     shuriken_attack_handle: Handle<AudioSource>,
// }

// #[derive(Resource)]
// pub struct CollisionSound(pub Handle<KiraAudioSource>);

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraAudioPlugin)
            // .add_audio_channel::<MusicChannel>()
            // .add_audio_channel::<EffectsChannel>()
            .add_startup_system(load_audio.label(GameSystemLabel::Core))
            // .add_startup_system_to_stage(
            //     StartupStage::PreStartup,
            //     load_audio.label(GameSystemLabel::Core),
            // )
            .add_system_set(
                SystemSet::new()
                    .with_system(collision_system)
                    .with_system(play_collision_sound.after(collision_system))
                    .label(GameSystemLabel::Core),
            );
    }
}

pub fn set_audio_channel_volume(
    music_channel: Res<AudioChannel<MusicChannel>>,
    effects_channel: Res<AudioChannel<EffectsChannel>>,
) {
    music_channel.set_volume(0.5);
    effects_channel.set_volume(0.5);
}

pub fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let collision_sound = asset_server.load("enemy/impact.ogg");
    let death_sound = asset_server.load("effects/disintegrate.ogg");
    let shuriken_sound = asset_server.load("player/swoosh.ogg");
    commands.insert_resource(SFXHandles {
        collision_sound,
        death_sound,
        shuriken_sound,
    });

    // let enemy_death_handle = asset_server.load("effects/disintegrate.ogg");
    // let shuriken_attack_handle = asset_server.load("player/swoosh.ogg");

    // commands.insert_resource(AudioSate {
    //     enemy_death_handle: enemy_death_handle,
    //     shuriken_attack_handle: shuriken_attack_handle,
    // })
}

fn play_collision_sound(
    audio: Res<KiraAudio>,
    sound: Res<SFXHandles>,
    mut collision_events: EventReader<SFXEvents>,
) {
    for event in collision_events.iter() {
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

    if !collision_events.is_empty() {
        collision_events.clear();
    }
}
