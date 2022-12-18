use crate::{assets::GameAssets, prelude::*, run_after_bonus_stage_intro};

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
    pub fn new(game_assets: Res<GameAssets>) -> Self {
        PlayerBundle {
            player: Player(PlayerAction::Idle, PlayerState::Intro),
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
                texture_atlas: game_assets.musashi.clone(),
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
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_run_criteria(run_after_bonus_stage_intro)
                .with_system(player_attacking_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_system(player_controls)
                .with_system(player_walking_animation)
                .with_system(player_flipping_animation),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_system(spawn_player),
        );
    }
}
pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(PlayerBundle::new(game_assets));
}

pub fn player_controls(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform, &Dimensions), With<Player>>,
) {
    let (mut player, mut player_transform, dimensions) = query.single_mut();
    let Bounds {
        top,
        right,
        bottom,
        left,
    } = calculate_bounds(&player_transform, Some(dimensions.0));

    if player.0 == PlayerAction::Falling || player.0 == PlayerAction::Attacking {
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
            player.0 = PlayerAction::Attacking;
        }
    }

    if player.1 == PlayerState::Intro {
        if keyboard_input.just_pressed(KeyCode::C) {
            player.0 = PlayerAction::Flipping;
        }

        if player.0 != PlayerAction::Flipping {
            if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
                player.0 = PlayerAction::WalkingLeft;
                if left > LEFT_WALL {
                    player_transform.translation.x -= WALKING_SPEED * time.delta().as_secs_f32();
                }
            }
            if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
                player.0 = PlayerAction::WalkingRight;
                if right < RIGHT_WALL {
                    player_transform.translation.x += WALKING_SPEED * time.delta().as_secs_f32();
                }
            } else if keyboard_input.any_just_released(vec![
                KeyCode::Right,
                KeyCode::D,
                KeyCode::Left,
                KeyCode::A,
            ]) {
                player.0 = PlayerAction::Idle;
            }
        }
    }
}

pub fn player_attacking_system(
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
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

    if player.0 == PlayerAction::Falling {
        sprite.index = 0;
        sprite.flip_x = false;
    }

    if player.0 != PlayerAction::Attacking {
        return;
    }

    sprite.flip_x = false;
    if sprite.index == 3 {
        player.0 = PlayerAction::Falling;
    }

    if attacking_timer.0.tick(time.delta()).just_finished() {
        sprite.index = (sprite.index + 1) % 4;

        if sprite.index == 3 {
            commands
                .spawn_empty()
                .insert(ShurikenBundle::new(game_assets, transform.translation));
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
) {
    let (player, mut walking_animation_timer, mut sprite) = query.single_mut();

    if player.0 != PlayerAction::Idle
        && player.0 != PlayerAction::WalkingLeft
        && player.0 != PlayerAction::WalkingRight
    {
        return;
    }

    if player.0 == PlayerAction::Idle {
        sprite.index = 7;
        return;
    }

    if player.0 == PlayerAction::WalkingLeft {
        sprite.flip_x = true;
    } else if player.0 == PlayerAction::WalkingRight {
        sprite.flip_x = false;
    }

    if walking_animation_timer.0.tick(time.delta()).just_finished() {
        sprite.index = sprite.index + 1;
    }

    if (player.0 == PlayerAction::WalkingLeft || player.0 == PlayerAction::WalkingRight)
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
    mut bonus_stage_events: ResMut<BonusStageEvents>,
) {
    let (mut player, mut flipping_animation_timer, mut sprite) = query.single_mut();

    if player.0 != PlayerAction::Flipping {
        return;
    }

    if player.0 == PlayerAction::Flipping && sprite.index < 14 {
        sprite.index = 14;
    }

    if flipping_animation_timer.tick(time.delta()).just_finished() {
        sprite.index = sprite.index + 1;
        if sprite.index > 19 {
            player.0 = PlayerAction::Falling;
            player.1 = PlayerState::Main;
            *bonus_stage_events = BonusStageEvents::Start;
        }
    }
}
