use bevy::prelude::*;

use crate::{
    audio::SFXEvents,
    components::{AttackingTimer, Player, PlayerState},
    shuriken::ShurikenBundle,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    attacking_timer: AttackingTimer,
    sprite_bundle: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let texture_handle = asset_server.load("player/joe_musashi_falling.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(45.0, 45.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        PlayerBundle {
            player: Player(PlayerState::Falling),
            attacking_timer: AttackingTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 140.0, 2.0),
                    ..Default::default()
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

    if player.0 != PlayerState::Attacking {
        return;
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

        if sprite.index == 0 {
            player.0 = PlayerState::Falling
        }
    }
}
