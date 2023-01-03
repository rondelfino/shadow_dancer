use crate::prelude::*;

pub struct TransitionPlugin;
impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Transition).with_system(initialize_movie), // .after(play_title_screen_music),
        )
        .add_system_set(SystemSet::on_update(GameState::Transition).with_system(transition));
    }
}

#[derive(Component)]
pub struct Transition;

pub struct MovieKeyframe {
    pub frames: Vec<usize>,
    pub timer: Timer,
    pub index: usize,
}

#[derive(Component)]
pub struct AnimationClip2D {
    pub keyframes: Vec<MovieKeyframe>,
    pub index: usize,
}

#[derive(Bundle)]
pub struct TransitionBundle {
    transition: Transition,
    sprite: SpriteSheetBundle,
    animation_clip: AnimationClip2D,
}

impl TransitionBundle {
    pub fn new(game_assets: Res<GameAssets>) -> Self {
        let scale = (WORLD_HEIGHT / 224.0) * CAMERA_SCALE;
        TransitionBundle {
            transition: Transition,
            sprite: SpriteSheetBundle {
                texture_atlas: game_assets.transition.clone(),
                transform: Transform::from_scale(Vec3::splat(scale)),
                ..Default::default()
            },

            animation_clip: AnimationClip2D {
                index: 0,
                keyframes: vec![
                    MovieKeyframe {
                        frames: (0..8).collect(),
                        timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (8..9).collect(),
                        timer: Timer::new(Duration::from_millis(40), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (9..10).collect(),
                        timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (10..13).collect(),
                        timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (13..23).collect(),
                        timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (23..24).collect(),
                        timer: Timer::new(Duration::from_millis(70), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (24..=30).collect(),
                        timer: Timer::new(Duration::from_millis(40), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (31..31).collect(),
                        timer: Timer::new(Duration::from_millis(260), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (31..37).collect(),
                        timer: Timer::new(Duration::from_millis(40), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (37..38).collect(),
                        timer: Timer::new(Duration::from_millis(290), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (38..92).collect(),
                        timer: Timer::new(Duration::from_millis(20), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (93..93).collect(),
                        timer: Timer::new(Duration::from_millis(2440), TimerMode::Repeating),
                        index: 0,
                    },
                    MovieKeyframe {
                        frames: (92..=156).collect(),
                        timer: Timer::new(Duration::from_millis(30), TimerMode::Repeating),
                        index: 0,
                    },
                ],
            },
        }
    }
}

pub fn load(asset_handler: &mut AssetHandler, game_assets: &mut ResMut<GameAssets>) {
    asset_handler.add_texture_atlas(
        &mut game_assets.transition,
        "intro/transition.png",
        Vec2::new(322.0, 226.0),
        11,
        15,
    );
    asset_handler.add_audio(&mut game_assets.melee_attack_sound, "sfx/melee_attack.ogg");
    asset_handler.add_audio(
        &mut game_assets.title_screen_bgm,
        "music/02 - Title - Keisuke Tsukahara.ogg",
    )
}

pub fn transition(
    time: Res<Time>,
    mut game_assets: ResMut<GameAssets>,
    mut asset_handler: AssetHandler,
    mut transition_query: Query<(&mut TextureAtlasSprite, &mut AnimationClip2D), With<Transition>>,
    mut bgm_events: EventWriter<BGMEvents>,
) {
    for (mut texture_atlas, mut animation_clip) in transition_query.iter_mut() {
        let animation_clip_index = animation_clip.index;
        let keyframe = animation_clip.keyframes.get_mut(animation_clip_index);

        if let Some(keyframe) = keyframe {
            if keyframe.timer.tick(time.delta()).just_finished() {
                let keyframe_frame = keyframe.frames.get(keyframe.index);
                match keyframe_frame {
                    None => {
                        animation_clip.index += 1;
                    }
                    Some(keyframe_frame) => {
                        texture_atlas.index = *keyframe_frame;
                        keyframe.index += 1;
                    }
                }
                if animation_clip.index >= 13 {
                    asset_handler.load(GameState::TitleScreen, &mut game_assets);
                }
                if texture_atlas.index == 6 {
                    bgm_events.send(BGMEvents::TitleScreenMusic);
                }
            }
        }
    }
}

pub fn initialize_movie(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut sfx_events: EventWriter<SFXEvents>,
) {
    commands.spawn(TransitionBundle::new(game_assets));
    sfx_events.send(SFXEvents::MeleeAttackSound);
}
