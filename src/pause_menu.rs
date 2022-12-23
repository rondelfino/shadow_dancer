use crate::prelude::*;

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Paused).with_system(pause_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Paused).with_system(despawner::<OnPauseScreen>),
            )
            .add_system(pause_game);
    }
}

#[derive(Component)]
struct OnPauseScreen;

// #[derive(Clone, Eq, PartialEq, Debug, Hash)]
// enum MenuState {
//     Main,
//     Settings,
//     SettingsDisplay,
//     SettingsSound,
//     Disabled,
// }

fn pause_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            OnPauseScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font: game_assets.menu_font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn pause_game(
    mut game_state: ResMut<State<GameState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut pause_event: ResMut<PauseEvent>,
) {
    if keyboard_input.any_pressed(vec![KeyCode::Escape, KeyCode::P]) {
        if *pause_event == PauseEvent::Unpaused {
            game_state.push(GameState::Paused).unwrap();
            keyboard_input.reset(KeyCode::Escape);
            *pause_event = PauseEvent::Paused;
        } else {
            game_state.pop().unwrap();
            keyboard_input.reset(KeyCode::Escape);
            *pause_event = PauseEvent::Unpaused;
        }
    }
}
