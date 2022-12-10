use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    attacking_timer: AttackingTimer,
    sprite_bundle: SpriteSheetBundle,
    walking_animation_timer: WalkingAnimationTimer,
    flipping_animation_timer: FlippingAnimationTimer,
    dimensions: Dimensions,
}

impl PlayerBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let texture_handle = asset_server.load("player/joe_musashi.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 7, 3, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        PlayerBundle {
            player: Player(PlayerState::Idle),
            attacking_timer: AttackingTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            walking_animation_timer: WalkingAnimationTimer(Timer::from_seconds(
                0.2,
                TimerMode::Repeating,
            )),
            flipping_animation_timer: FlippingAnimationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 2.0),
                    ..default()
                },
                ..default()
            },
            dimensions: Dimensions(Vec2::new(42.0, 42.0)),
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_controls.label(GameSystemLabel::Core))
            .add_system_set(
                SystemSet::on_update(GameState::StageIntro)
                    .label(GameSystemLabel::Core)
                    .with_system(player_walking_animation)
                    .with_system(player_flipping_animation),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(GameSystemLabel::Core)
                    .with_system(player_attacking_system),
            );
    }
}
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_empty()
        .insert(PlayerBundle::new(asset_server, texture_atlases));
}

pub fn player_controls(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Player,
            &mut Transform,
            &TextureAtlasSprite,
            &Dimensions,
        ),
        With<Player>,
    >,
) {
    let (mut player, mut player_transform, sprite, dimensions) = query.single_mut();
    let Bounds {
        top,
        right,
        bottom,
        left,
    } = calculate_bounds(&player_transform, Some(dimensions.0));

    if player.0 == PlayerState::Falling || player.0 == PlayerState::Attacking {
        if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) && left > LEFT_WALL {
            player_transform.translation.x -= PLAYER_AIR_SPEED * time.delta().as_secs_f32();
        }

        if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) && right < RIGHT_WALL {
            player_transform.translation.x += PLAYER_AIR_SPEED * time.delta().as_secs_f32();
        }

        if keyboard_input.pressed(KeyCode::W) && top < UPPER_BOUND {
            player_transform.translation.y += PLAYER_AIR_SPEED * time.delta().as_secs_f32();
        }

        if keyboard_input.pressed(KeyCode::S) && bottom > LOWER_BOUND {
            player_transform.translation.y -= PLAYER_AIR_SPEED * time.delta().as_secs_f32();
        }

        if keyboard_input.any_just_pressed(vec![KeyCode::Down, KeyCode::X]) {
            player.0 = PlayerState::Attacking;
        }
    }

    if game_state.current() == &GameState::StageIntro && player.0 != PlayerState::Flipping {
        if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
            player.0 = PlayerState::WalkingLeft;
            if left > LEFT_WALL {
                player_transform.translation.x -= WALKING_SPEED * time.delta().as_secs_f32();
            }
        }
        if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
            player.0 = PlayerState::WalkingRight;
            if right < RIGHT_WALL {
                player_transform.translation.x += WALKING_SPEED * time.delta().as_secs_f32();
            }
        } else if keyboard_input.any_just_released(vec![
            KeyCode::Right,
            KeyCode::D,
            KeyCode::Left,
            KeyCode::A,
        ]) {
            player.0 = PlayerState::Idle;
        }
    }

    if keyboard_input.just_pressed(KeyCode::C) && game_state.current() != &GameState::InGame {
        player.0 = PlayerState::Flipping;
    }
}

pub fn player_attacking_system(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut Player,
            &mut AttackingTimer,
            &Transform,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    mut sfx_events: EventWriter<SFXEvents>,
) {
    let (mut player, mut attacking_timer, transform, mut sprite) = query.single_mut();

    if player.0 == PlayerState::Falling {
        sprite.index = 0;
        sprite.flip_x = false;
    }

    if player.0 != PlayerState::Attacking {
        return;
    }

    sprite.flip_x = false;
    if sprite.index == 3 {
        player.0 = PlayerState::Falling;
    }

    if attacking_timer.0.tick(time.delta()).just_finished() {
        sprite.index = (sprite.index + 1) % 4;

        if sprite.index == 3 {
            commands.spawn_empty().insert(ShurikenBundle::new(
                asset_server,
                texture_atlases,
                transform.translation,
            ));
            sfx_events.send(SFXEvents::ShurikenSound);
        }
    }
}

pub fn player_walking_animation(
    time: Res<Time>,

    mut query: Query<
        (
            &mut Player,
            &mut WalkingAnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    mut sfx_events: EventWriter<SFXEvents>,
    game_state: Res<State<GameState>>,
) {
    let (mut player, mut walking_animation_timer, mut sprite) = query.single_mut();

    if player.0 != PlayerState::Idle
        && player.0 != PlayerState::WalkingLeft
        && player.0 != PlayerState::WalkingRight
    {
        return;
    }

    if game_state.current() == &GameState::InGame {
        player.0 = PlayerState::Falling;
    }

    if player.0 == PlayerState::Idle {
        sprite.index = 7;
        return;
    }

    if player.0 == PlayerState::WalkingLeft {
        sprite.flip_x = true;
    } else if player.0 == PlayerState::WalkingRight {
        sprite.flip_x = false;
    }

    if walking_animation_timer.0.tick(time.delta()).just_finished() {
        sprite.index = sprite.index + 1;
    }

    if (player.0 == PlayerState::WalkingLeft || player.0 == PlayerState::WalkingRight)
        && (sprite.index > 13 || sprite.index < 8)
    {
        sprite.index = 8;
    }
}

pub fn player_flipping_animation(
    time: Res<Time>,

    mut query: Query<
        (
            &mut Player,
            &mut FlippingAnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    mut sfx_events: EventWriter<SFXEvents>,
    mut game_state: ResMut<State<GameState>>,
) {
    let (mut player, mut flipping_animation_timer, mut sprite) = query.single_mut();

    if player.0 != PlayerState::Flipping {
        return;
    }

    if player.0 == PlayerState::Flipping && sprite.index < 14 {
        sprite.index = 14;
    }

    if flipping_animation_timer.tick(time.delta()).just_finished() {
        sprite.index = sprite.index + 1;
        if sprite.index > 19 {
            game_state.set(GameState::InGame).unwrap();
            player.0 = PlayerState::Falling;
        }
    }
}
