use crate::{asset_loading::AssetHandler, prelude::*};

#[derive(Default, Debug, Resource)]
pub struct GameAssets {
    pub musashi: Handle<TextureAtlas>,
    pub red_ninja: Handle<TextureAtlas>,
    pub death_effect: Handle<TextureAtlas>,

    pub shuriken: Handle<Image>,

    pub background: Handle<Image>,
    pub left_roof: Handle<Image>,
    pub right_roof: Handle<Image>,
    pub right_wall: Handle<Image>,
    pub left_wall: Handle<Image>,

    pub collision_sound: Handle<KiraAudioSource>,
    pub death_sound: Handle<KiraAudioSource>,
    pub shuriken_sound: Handle<KiraAudioSource>,
    pub reflection_sound: Handle<KiraAudioSource>,
    pub melee_attack_sound: Handle<KiraAudioSource>,

    pub ingame_bgm: Handle<KiraAudioSource>,
    pub title_screen_bgm: Handle<KiraAudioSource>,

    pub menu_font: Handle<Font>,

    pub splash_screen: Handle<Image>,
    pub transition: Handle<TextureAtlas>,
}

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default());
    }
}

impl AssetsPlugin {
    pub fn load(asset_handler: &mut AssetHandler, game_assets: &mut ResMut<GameAssets>) {
        println!("loading for ingame");
        asset_handler.add_texture_atlas(
            &mut game_assets.musashi,
            "sprites/player/joe_musashi.png",
            Vec2 { x: 64.0, y: 64.0 },
            7,
            3,
        );
        asset_handler.add_texture_atlas(
            &mut game_assets.red_ninja,
            "sprites/enemy/red_ninja.png",
            Vec2::new(40.0, 65.0),
            4,
            1,
        );
        asset_handler.add_texture_atlas(
            &mut game_assets.death_effect,
            "sprites/effects/death.png",
            Vec2::new(40.0, 95.0),
            4,
            1,
        );

        asset_handler.add_sprites(&mut game_assets.shuriken, "objects/shuriken.png");

        asset_handler.add_sprites(&mut game_assets.background, "background/background_day.png");
        asset_handler.add_sprites(&mut game_assets.left_roof, "background/left_roof.png");
        asset_handler.add_sprites(&mut game_assets.right_roof, "background/right_roof.png");
        asset_handler.add_sprites(&mut game_assets.right_wall, "background/right_wall.png");
        asset_handler.add_sprites(&mut game_assets.left_wall, "background/left_wall.png");

        asset_handler.add_audio(&mut game_assets.collision_sound, "sfx/impact.ogg");
        asset_handler.add_audio(&mut game_assets.death_sound, "sfx/disintegrate.ogg");
        asset_handler.add_audio(&mut game_assets.shuriken_sound, "sfx/shuriken.ogg");
        asset_handler.add_audio(&mut game_assets.reflection_sound, "sfx/parry.ogg");
        asset_handler.add_audio(&mut game_assets.melee_attack_sound, "sfx/melee_attack.ogg");

        asset_handler.add_audio(
            &mut game_assets.ingame_bgm,
            "music/08 - Bonus Stage - Keisuke Tsukahara.ogg",
        );

        asset_handler.add_font(
            &mut game_assets.menu_font,
            "ui/shadow-dancer-the-secret-of-shinobi-smd.ttf",
        );
    }
}
