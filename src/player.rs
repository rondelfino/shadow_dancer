use bevy::prelude::*;

use crate::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let texture_handle = asset_server.load("player/joe_musashi_falling.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(45.0, 45.0), 4, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        PlayerBundle {
            player: Player,
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..default()
            },
        }
    }
}
