use crate::prelude::*;

pub struct TitleScreenPlugin;
impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleTimer>()
            .add_system_set(
                SystemSet::on_enter(GameState::TitleScreen).with_system(title_screen_setup),
            )
            .add_system_set(SystemSet::on_update(GameState::TitleScreen).with_system(flash_text))
            .add_system_set(
                SystemSet::on_exit(GameState::TitleScreen)
                    .with_system(despawner::<OnTitleScreen>)
                    .with_system(despawner::<Transition>),
            );
    }
}

#[derive(Component)]
pub struct OnTitleScreen;

pub fn load(asset_handler: &mut AssetHandler, game_assets: &mut ResMut<GameAssets>) {
    asset_handler.add_font(
        &mut game_assets.menu_font,
        "ui/shadow-dancer-the-secret-of-shinobi-smd.ttf",
    );
    asset_handler.add_audio(
        &mut game_assets.title_screen_bgm,
        "music/02 - Title - Keisuke Tsukahara.ogg",
    );
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
            commands.spawn(title_text.clone());
        });
}

pub fn flash_text(
    mut query: Query<&mut Visibility, With<OnTitleScreen>>,
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
