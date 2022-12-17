use crate::{assets::GameAssets, prelude::*};

pub struct GameAudioPlugin;

#[derive(Resource)]
pub struct BGMChannel;

#[derive(Resource)]
pub struct SFXChannel;

pub enum SFXEvents {
    CollisionSound,
    DeathSound,
    ShurikenSound,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraAudioPlugin)
            .add_audio_channel::<BGMChannel>()
            .add_audio_channel::<SFXChannel>()
            .add_event::<SFXEvents>()
            .add_startup_system_to_stage(StartupStage::PreStartup, set_audio_channel_volume)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(play_bgm))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
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

fn play_sfx(
    audio: Res<AudioChannel<SFXChannel>>,
    game_assets: Res<GameAssets>,
    mut sfx_events: EventReader<SFXEvents>,
) {
    for event in sfx_events.iter() {
        match event {
            SFXEvents::CollisionSound => {
                audio.play(game_assets.collision_sound.clone());
            }
            SFXEvents::DeathSound => {
                audio.play(game_assets.death_sound.clone());
            }
            SFXEvents::ShurikenSound => {
                audio.play(game_assets.shuriken_sound.clone());
            }
        }
    }

    if !sfx_events.is_empty() {
        sfx_events.clear();
    }
}

pub fn play_bgm(audio: Res<AudioChannel<BGMChannel>>, game_assets: Res<GameAssets>) {
    audio
        .play(game_assets.bgm_01.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        .looped();
}
