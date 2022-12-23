use crate::{pause_game, prelude::*};

#[derive(Bundle)]
pub struct BackgroundBundle {
    background: Background,
    dimensions: Dimensions,
    sprite_bundle: SpriteBundle,
}

#[derive(Resource, Debug)]
pub struct BonusStageTimer(pub Timer);

impl Default for BonusStageTimer {
    fn default() -> Self {
        BonusStageTimer(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

impl BackgroundBundle {
    pub fn new(game_assets: Res<GameAssets>) -> Self {
        let dimensions = Dimensions(Vec2::new(1248.0, 1667.0));

        let scale = WORLD_WIDTH / (dimensions.0.x - 207.0);
        let sprite_bundle = SpriteBundle {
            texture: game_assets.background.clone(),
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    -((dimensions.0.y * scale) / 2.0) + (WORLD_HEIGHT / 4.0) + 55.0,
                    0.0,
                ),
                scale: Vec3::new(scale, scale, 0.0),
                ..default()
            },
            ..default()
        };
        BackgroundBundle {
            background: Background,
            dimensions,
            sprite_bundle,
        }
    }
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BonusStageTimer>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_background))
            .add_system_set(SystemSet::on_enter(GameState::EndStage).with_system(end_stage_background_fadeout))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_run_criteria(pause_game)
                    .with_system(background_animator)
                    .with_system(bonus_stage_transition),
            );
    }
}

pub fn spawn_background(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(BackgroundBundle::new(game_assets));
}

pub fn background_animator(mut query: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    for mut background_transform in query.iter_mut() {
        background_transform.translation.y += (FALLING_SPEED / 100.0) * time.delta_seconds();
    }
}

pub fn bonus_stage_transition(
    time: Res<Time>,
    mut stopwatch: ResMut<BonusStageTimer>,
    mut query: Query<&Player>,
    mut game_state: ResMut<State<GameState>>,
    pause_event: Res<PauseEvent>,
) {
    let player = query.single_mut();

    if player.1 == LevelState::Start && *pause_event == PauseEvent::Unpaused {
        stopwatch.0.tick(time.delta());
    }

    if stopwatch.0.just_finished() {
        game_state.set(GameState::EndStage).unwrap();
    }
}

pub fn end_stage_background_fadeout(mut commands: Commands) {
    println!("ending");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),

                ..Default::default()
            },
            ..Default::default()
        },
        Sprite {
            color: Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            },
            custom_size: Some(Vec2::new(WORLD_WIDTH * 2.0, WORLD_HEIGHT)),
            ..Default::default()
        }
        .ease_to(
            Sprite {
                custom_size: Some(Vec2::new(WORLD_WIDTH * 2.0, WORLD_HEIGHT)),
                color: Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                ..Default::default()
            },
            EaseFunction::ExponentialIn,
            EasingType::Once {
                duration: std::time::Duration::from_secs(1),
            },
        ),
    ));
}
