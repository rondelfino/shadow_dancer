use crate::{assets::GameAssets, pause_game, prelude::*};

pub const PLAYER_WALKING_SPEED: f32 = 150.0;
pub const PLAYER_AIR_SPEED: f32 = 350.0;
pub const TERMINAL_VELOCITY: f32 = 30.0;
pub const PLAYER_FLIPPING_SPEED: f32 = 250.0;
pub const FINAL_PLAYER_POS_Y: f32 = -100.0;
pub const BONUS_STAGE_ENDING_PLAYER_STARTING_POS_X: f32 = 0.0;
pub const BONUS_STAGE_ENDING_PLAYER_STARTING_POS_Y: f32 = 100.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    attacking_timer: AttackingTimer,
    sprite_bundle: SpriteSheetBundle,
    walking_animation_timer: WalkingAnimationTimer,
    flipping_animation_timer: FlippingAnimationTimer,
    dimensions: Dimensions,
    gravity: Gravity,
    velocity: Velocity,
}

impl PlayerBundle {
    pub fn new(game_assets: Res<GameAssets>) -> Self {
        PlayerBundle {
            player: Player(PlayerAction::Idle, LevelState::Intro),
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
                    translation: Vec3::new(BONUS_STAGE_SPAWN_POS.x, BONUS_STAGE_SPAWN_POS.y, 2.0),
                    ..default()
                },
                ..default()
            },
            dimensions: Dimensions(Vec2::new(42.0, 42.0)),
            gravity: Gravity(1.75),
            velocity: Velocity(Vec2::new(0.0, PLAYER_FLIPPING_SPEED)),
        }
    }
}
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(pause_game)
                .with_system(player_attacking_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_controls)
                .with_system(player_walking_animation)
                .with_system(player_flipping_animation),
        )
        .add_system_set(
            SystemSet::on_update(GameState::EndStage)
                .with_system(player_scoreboard_animation)
                .with_system(end_stage_movement)
                .with_system(despawner::<Shuriken>)
                .with_system(despawner::<Effect>),
        )
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_player));
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

    if player.1 == LevelState::Intro {
        if keyboard_input.just_pressed(KeyCode::C) {
            player.0 = PlayerAction::Flipping;
        }

        if player.0 != PlayerAction::Flipping {
            if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
                player.0 = PlayerAction::WalkingLeft;
                if left > LEFT_WALL {
                    player_transform.translation.x -=
                        PLAYER_WALKING_SPEED * time.delta().as_secs_f32();
                }
            }
            if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
                player.0 = PlayerAction::WalkingRight;
                if right < BONUS_STAGE_INTRO_RIGHT_BOUNDARY {
                    player_transform.translation.x +=
                        PLAYER_WALKING_SPEED * time.delta().as_secs_f32();
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
    if sprite.index == 4 {
        player.0 = PlayerAction::Falling;
    }

    attacking_timer.0.tick(time.delta());
    if attacking_timer.0.just_finished() {
        sprite.index = (sprite.index + 1) % 6;
        println!("{:?}", sprite.index);

        if sprite.index == 4 {
            commands.spawn_empty().insert(ShurikenBundle::new(
                game_assets,
                Vec3::new(
                    transform.translation.x,
                    transform.translation.y - 20.0,
                    transform.translation.z,
                ),
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
            &mut Transform,
            &Gravity,
            &mut Velocity,
        ),
        With<Player>,
    >,
    mut sfx_events: EventWriter<SFXEvents>,
    mut bonus_stage_events: ResMut<PauseEvent>,
) {
    for (
        mut player,
        mut flipping_animation_timer,
        mut sprite,
        mut transform,
        gravity,
        mut velocity,
    ) in query.iter_mut()
    {
        if player.0 != PlayerAction::Flipping {
            return;
        }

        if player.0 == PlayerAction::Flipping && sprite.index < 14 {
            sprite.index = 14;
        }

        if sprite.index == 15 {
            if transform.translation.y < TERMINAL_VELOCITY {
                transform.translation.y +=
                    time.delta_seconds() * (velocity.y + time.delta_seconds() * gravity.0 / 2.0);
                velocity.y += gravity.0 * time.delta_seconds();

                continue;
            }
        } else if sprite.index > 15 {
            transform.translation.y -=
                time.delta_seconds() * (velocity.y + time.delta_seconds() * gravity.0 / 2.0);
            velocity.y -= gravity.0 * time.delta_seconds();
        }

        if flipping_animation_timer.tick(time.delta()).just_finished() {
            sprite.index = sprite.index + 1;
            if sprite.index > 19 {
                player.0 = PlayerAction::Falling;
                player.1 = LevelState::Start;
                *bonus_stage_events = PauseEvent::Unpaused;
            }
        }
    }
}

pub fn player_scoreboard_animation(
    time: Res<Time>,

    mut query: Query<
        (
            &mut Player,
            &mut FlippingAnimationTimer,
            &mut TextureAtlasSprite,
            &mut Transform,
            &Gravity,
            &mut Velocity,
        ),
        With<Player>,
    >,
    mut sfx_events: EventWriter<SFXEvents>,
) {
    for (
        mut player,
        mut flipping_animation_timer,
        mut sprite,
        mut transform,
        gravity,
        mut velocity,
    ) in query.iter_mut()
    {
        if player.0 == PlayerAction::Landing && sprite.index < 14 {
            sprite.index = 19;
        }

        if player.0 == PlayerAction::Landed {
            sprite.index = 14;
            return;
        }

        if sprite.index == 15 {
            if transform.translation.y.round() >= FINAL_PLAYER_POS_Y {
                println!("{:?}", transform.translation.y.round());
                transform.translation.y -=
                    time.delta_seconds() * (velocity.y + time.delta_seconds() * gravity.0 / 2.0);
                velocity.y -= gravity.0 * time.delta_seconds();
            } else {
                player.0 = PlayerAction::Landed;
            }
            return;
        }

        if sprite.index > 15 {
            transform.translation.y -=
                time.delta_seconds() * (velocity.y + time.delta_seconds() * gravity.0 / 2.0);
            velocity.y -= gravity.0 * time.delta_seconds();
        }

        if flipping_animation_timer.tick(time.delta()).just_finished()
            && player.0 == PlayerAction::Landing
        {
            sprite.index = sprite.index - 1;

            println!("{:?}", sprite.index);
        }
    }
}

fn end_stage_movement(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        if player.0 == PlayerAction::Landing || player.0 == PlayerAction::Landed {
            return;
        }
        if transform.translation.x.round() < BONUS_STAGE_ENDING_PLAYER_STARTING_POS_X {
            transform.translation.x += PLAYER_WALKING_SPEED * time.delta_seconds();
        } else if transform.translation.x.round() > BONUS_STAGE_ENDING_PLAYER_STARTING_POS_X {
            transform.translation.x -= PLAYER_WALKING_SPEED * time.delta_seconds();
        }

        if transform.translation.y.round() < BONUS_STAGE_ENDING_PLAYER_STARTING_POS_Y {
            transform.translation.y += PLAYER_WALKING_SPEED * time.delta_seconds();
        } else if transform.translation.y.round() > BONUS_STAGE_ENDING_PLAYER_STARTING_POS_Y {
            transform.translation.y -= PLAYER_WALKING_SPEED * time.delta_seconds();
        }

        if transform.translation.y.round() == BONUS_STAGE_ENDING_PLAYER_STARTING_POS_Y
            && transform.translation.x.round() == BONUS_STAGE_ENDING_PLAYER_STARTING_POS_X
        {
            player.0 = PlayerAction::Landing;
        }
    }
}
