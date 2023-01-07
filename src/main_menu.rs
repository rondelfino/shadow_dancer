use bevy::app::AppExit;

use crate::prelude::*;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentSelection { index: 0 })
            .add_event::<NavigationEvent>()
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(main_menu_setup))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(handle_current_selection)
                    .with_system(keyboard_input_system)
                    .with_system(navigate_menu),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(despawner::<OnMainMenuScreen>),
            );
    }
}

#[derive(Component, Debug)]
pub struct Selectables {
    key: usize,
}

#[derive(Resource, Debug)]
pub struct CurrentSelection {
    pub index: usize,
}
#[derive(Component, Debug)]
pub struct MenuButton;

#[derive(Component, Debug)]
pub struct MenuSelector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationEvent {
    Up,
    Down,
    Select,
}

#[derive(Bundle)]
pub struct MenuBundle {
    button: Button,
    text_bundle: TextBundle,
    selectable: Selectables,
}

#[derive(Component, Debug)]
pub struct TextColor {
    color: Color,
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor {
            color: Color::WHITE,
        }
    }
}

impl MenuBundle {
    pub fn new(text: &str, index: usize, font: Handle<Font>, text_color: TextColor) -> Self {
        MenuBundle {
            button: Button,
            text_bundle: TextBundle::from_section(
                text,
                TextStyle {
                    font: font,
                    font_size: 24.0,
                    color: text_color.color,
                },
            )
            .with_style(Style {
                justify_content: JustifyContent::FlexStart,
                align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(25.0)),
                size: Size::new(Val::Px(100.0), Val::Px(20.0)),
                ..Default::default()
            }),
            selectable: Selectables { key: index },
        }
    }
}

pub fn load(asset_handler: &mut AssetHandler, game_assets: &mut ResMut<GameAssets>) {
    asset_handler.add_font(
        &mut game_assets.menu_font,
        "ui/shadow-dancer-the-secret-of-shinobi-smd.ttf",
    );
    asset_handler.add_sprites(&mut game_assets.menu_arrow, "ui/menu_arrow.png");
    asset_handler.add_audio(&mut game_assets.menu_sfx, "ui/menu_sfx.ogg");
}

#[derive(Component)]
struct OnMainMenuScreen;

fn main_menu_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    MenuBundle::new(
                        "Start",
                        0,
                        game_assets.menu_font.clone(),
                        TextColor::default(),
                    ),
                    MenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage(game_assets.menu_arrow.clone()),
                            style: Style {
                                
                                position: UiRect {
                                    right: Val::Px(35.0),
                                    top: Val::Px(5.0),
                                    ..Default::default()
                                },
                                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        MenuSelector,
                    ));
                });

            parent.spawn((
                MenuBundle::new(
                    "Quit",
                    1,
                    game_assets.menu_font.clone(),
                    TextColor::default(),
                ),
                MenuButton,
            ));
        });
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut nv_writer: EventWriter<NavigationEvent>,
    mut sfx_writer: EventWriter<SFXEvents>,
) {
    use NavigationEvent::*;
    if keyboard_input.just_pressed(KeyCode::Down) {
        nv_writer.send(Down);
        sfx_writer.send(SFXEvents::MenuSFX);
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        nv_writer.send(Up);
        sfx_writer.send(SFXEvents::MenuSFX);
    } else if keyboard_input.any_just_pressed(vec![KeyCode::Return, KeyCode::V]) {
        nv_writer.send(Select);
        sfx_writer.send(SFXEvents::MenuSFX);
    }
}

pub fn handle_current_selection(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Selectables, &mut Text), With<MenuButton>>,
    selector_query: Query<Entity, With<MenuSelector>>,
    current: ResMut<CurrentSelection>,
) {
    let selector = selector_query.single();
    for (entity, selectables, mut text) in query.iter_mut() {
        for section in text.sections.iter_mut() {
            if current.index == selectables.key {
                section.style.color = Color::RED;
                commands.entity(entity).add_child(selector);
            } else {
                section.style.color = Color::WHITE;
            }
        }
    }
}

pub fn navigate_menu(
    mut reader: EventReader<NavigationEvent>,
    mut current: ResMut<CurrentSelection>,
    mut asset_handler: AssetHandler,
    mut game_assets: ResMut<GameAssets>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for navigation_event in reader.iter() {
        if navigation_event == &NavigationEvent::Up {
            if current.index == 0 {
                current.index += 1;
            } else if current.index == 1 {
                current.index -= 1;
            }
        }

        if navigation_event == &NavigationEvent::Down {
            if current.index == 0 {
                current.index += 1;
            } else if current.index == 1 {
                current.index -= 1;
            }
        }

        if navigation_event == &NavigationEvent::Select {
            if current.index == 0 {
                asset_handler.load(GameState::LoadWorld, &mut game_assets);
            } else if current.index == 1 {
                app_exit_events.send(AppExit);
                current.index -= 1;

            }
        }
    }
}
