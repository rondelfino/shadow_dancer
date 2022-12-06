use bevy::prelude::*;

use crate::{
    audio::SFXEvents,
    components::{
        AttackingTimer, FlippingAnimationTimer, Player, PlayerState, WalkingAnimationTimer,
    },
    shuriken::ShurikenBundle,
    GameState,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    attacking_timer: AttackingTimer,
    sprite_bundle: SpriteSheetBundle,
    walking_animation_timer: WalkingAnimationTimer,
    flipping_animation_timer: FlippingAnimationTimer,
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
                    translation: Vec3::new(0.0, 50.0, 2.0),
                    ..default()
                },
                ..default()
            },
        }
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
    }

    if player.0 != PlayerState::Attacking {
        return;
    }

    if sprite.index == 3 {
        player.0 = PlayerState::Falling;
    }

    sprite.flip_x = false;

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
) {
    let (mut player, mut walking_animation_timer, mut sprite) = query.single_mut();

    if player.0 != PlayerState::Idle
        && player.0 != PlayerState::WalkingLeft
        && player.0 != PlayerState::WalkingRight
    {
        return;
    }

    println!("{:?}", player.0);

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
    let (mut player, mut jumping_animation_timer, mut sprite) = query.single_mut();

    if player.0 != PlayerState::Flipping {
        return;
    }

    if player.0 == PlayerState::Flipping && sprite.index < 14 {
        sprite.index = 14;
    }

    if jumping_animation_timer.tick(time.delta()).just_finished() {
        sprite.index = sprite.index + 1;
    }

    if sprite.index > 19 {
        player.0 = PlayerState::Falling;
        game_state.set(GameState::InGame).unwrap();
    }
}
