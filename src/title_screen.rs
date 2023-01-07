use bevy::input::keyboard::KeyboardInput;

use crate::prelude::*;

pub struct TitleScreenPlugin;
impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleTimer>()
            .add_system_set(
                SystemSet::on_enter(GameState::TitleScreen).with_system(title_screen_setup),
            )
            .add_system_set(
                SystemSet::on_update(GameState::TitleScreen)
                    .with_system(flash_text)
                    .with_system(next_state),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::TitleScreen)
                    .with_system(despawner::<OnTitleScreen>)
                    .with_system(despawner::<FlashingText>)
                    .with_system(despawner::<Transition>),
            );
    }
}

#[derive(Component)]
pub struct OnTitleScreen;

#[derive(Component)]
pub struct FlashingText;

pub fn load(asset_handler: &mut AssetHandler, game_assets: &mut ResMut<GameAssets>) {
    asset_handler.add_font(
        &mut game_assets.menu_font,
        "ui/shadow-dancer-the-secret-of-shinobi-smd.ttf",
    );
    asset_handler.add_audio(
        &mut game_assets.title_screen_bgm,
        "music/02 - Title - Keisuke Tsukahara.ogg",
    );
    asset_handler.add_sprites(&mut game_assets.title_screen, "intro/title_screen.png");
}

#[derive(Resource, Debug)]
pub struct TitleTimer(pub Timer);

impl Default for TitleTimer {
    fn default() -> Self {
        TitleTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn title_screen_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    println!("title screen");
    let text_color = Color::Rgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
    };

    let title_text = TextBundle::from_section(
        "Press Start Button",
        TextStyle {
            font: game_assets.menu_font.clone(),
            font_size: 25.0,
            color: text_color.clone(),
        },
    );

    let scale = (WORLD_HEIGHT / 224.0) * CAMERA_SCALE;

    commands.spawn((
        SpriteBundle {
            texture: game_assets.title_screen.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                scale: Vec3::splat(scale),
                ..Default::default()
            },
            ..Default::default()
        },
        OnTitleScreen,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px(205.0),
                        top: Val::Px(300.0),
                        bottom: Val::Px(105.0),
                        ..Default::default()
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnTitleScreen,
        ))
        .with_children(|commands| {
            commands.spawn((title_text.clone(), FlashingText));
        });
}

pub fn flash_text(
    mut query: Query<&mut Visibility, With<FlashingText>>,
    mut title_timer: ResMut<TitleTimer>,
    time: Res<Time>,
) {
    for mut text_visibility in query.iter_mut() {
        if title_timer.0.tick(time.delta()).finished() {
            if text_visibility.is_visible {
                *text_visibility = Visibility::INVISIBLE
            } else {
                *text_visibility = Visibility::VISIBLE
            }
        }
    }
}
pub fn next_state(
    mut game_assets: ResMut<GameAssets>,
    mut asset_handler: AssetHandler,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for _ in keyboard_input_events.iter() {
        asset_handler.load(GameState::MainMenu, &mut game_assets);
    }
}
