use bevy::{core_pipeline::clear_color::ClearColorConfig, input::keyboard::KeyboardInput};

use crate::prelude::*;

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(setup_splash_screen))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(next_state))
            .init_resource::<SplashTimer>()
            .add_system_set(
                SystemSet::on_exit(GameState::Splash).with_system(despawner::<OnSplashScreen>),
            );
    }
}

#[derive(Component, Debug)]
pub struct OnSplashScreen;

#[derive(Resource)]
pub struct SplashTimer(pub Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        SplashTimer(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

pub fn load(asset_handler: &mut AssetHandler, game_assets: &mut ResMut<GameAssets>) {
    asset_handler.add_sprites(&mut game_assets.splash_screen, "splash.png");
}

pub fn setup_splash_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    println!("splash starting");
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.0,
                scaling_mode: ScalingMode::FixedVertical(WORLD_HEIGHT),
                scale: CAMERA_SCALE,
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            },
            ..Default::default()
        },
        OnSplashScreen,
    ));

    let starting_sprite = Sprite {
        color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 0.0,
        },
        ..Default::default()
    };

    commands.spawn((
        SpriteBundle {
            texture: game_assets.splash_screen.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: starting_sprite.clone(),
            ..Default::default()
        },
        OnSplashScreen,
        starting_sprite.ease_to(
            Sprite {
                color: Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                ..Default::default()
            },
            EaseFunction::ExponentialIn,
            EasingType::PingPong {
                duration: std::time::Duration::from_secs(3),
                pause: Some(std::time::Duration::from_secs(3)),
            },
        ),
    ));
}

pub fn next_state(
    time: Res<Time>,
    mut splash_timer: ResMut<SplashTimer>,
    mut game_assets: ResMut<GameAssets>,
    mut asset_handler: AssetHandler,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    if splash_timer.0.tick(time.delta()).just_finished() {
        asset_handler.load(GameState::LoadWorld, &mut game_assets)
    }

    for _ in keyboard_input_events.iter() {
        asset_handler.load(GameState::LoadWorld, &mut game_assets)
    }
}
