use bevy::{asset::Asset, ecs::system::SystemParam};

use crate::{
    assets::{self, GameAssets},
    prelude::*,
    splash, title_screen, transition, main_menu,
};
use std::{marker::PhantomData, vec};

pub struct AssetsLoadingPlugin;
impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoading>()
            .init_resource::<NextState>()
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_assets));
    }
}

#[derive(Resource, Debug)]
pub struct NextState {
    state: GameState,
}

impl Default for NextState {
    fn default() -> Self {
        NextState {
            state: GameState::TitleScreen,
        }
    }
}

#[derive(Default)]
pub struct GameTexture {
    pub material: Handle<StandardMaterial>,
    pub image: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct AssetLoading {
    pub asset_handles: Vec<(HandleUntyped, String)>,
}

#[derive(SystemParam)]
pub struct AssetHandler<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    asset_loading: ResMut<'w, AssetLoading>,
    texture_atlases: ResMut<'w, Assets<TextureAtlas>>,
    state: ResMut<'w, State<GameState>>,
    next_state: ResMut<'w, NextState>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> AssetHandler<'w, 's> {
    fn add_asset<T: Asset>(&mut self, asset: &mut Handle<T>, path: &str) {
        *asset = self.asset_server.load(path);
        self.asset_loading
            .asset_handles
            .push((asset.clone_untyped(), path.to_string()));
    }

    pub fn load(&mut self, next_game_state: GameState, game_assets: &mut ResMut<GameAssets>) {
        self.queue_assets_for_state(&next_game_state, game_assets);
        self.next_state.state = next_game_state;
        self.state.set(GameState::Loading).unwrap();
    }

    pub fn add_sprites(&mut self, sprite: &mut Handle<Image>, path: &str) {
        self.add_asset(sprite, path)
    }

    pub fn add_texture_atlas(
        &mut self,
        texture_atlas_handle: &mut Handle<TextureAtlas>,
        path: &str,
        tile_size: Vec2,
        col: usize,
        row: usize,
    ) {
        let mut sprite: Handle<Image> = Handle::default();
        self.add_asset(&mut sprite, path);
        let texture_atlas =
            TextureAtlas::from_grid(sprite.clone(), tile_size, col, row, None, None);

        *texture_atlas_handle = self.texture_atlases.add(texture_atlas);
    }

    pub fn add_audio(&mut self, audio: &mut Handle<KiraAudioSource>, path: &str) {
        self.add_asset(audio, path);
    }

    pub fn add_font(&mut self, font: &mut Handle<Font>, path: &str) {
        self.add_asset(font, path);
    }

    fn queue_assets_for_state(
        &mut self,
        game_state: &GameState,
        game_assets: &mut ResMut<GameAssets>,
    ) {
        match game_state {
            GameState::Splash => splash::load(self, game_assets),
            GameState::Transition => transition::load(self, game_assets),
            GameState::TitleScreen => title_screen::load(self, game_assets),
            GameState::MainMenu => main_menu::load(self, game_assets),
            GameState::LoadWorld => assets::AssetsPlugin::load(self, game_assets),
            _ => (),
        }
    }
}

fn check_assets(mut asset_handler: AssetHandler) {
    use bevy::asset::LoadState;

    let mut ready = true;
    for (handle, path) in asset_handler.asset_loading.asset_handles.iter() {
        match asset_handler.asset_server.get_load_state(handle) {
            LoadState::Failed => {
                // one of our assets had an error
                panic!("An asset had an error: {:?} {:?}", handle, path);
            }

            LoadState::Loaded => {

                // all assets are now ready

                // this might be a good place to transition into your in-game state

                // remove the resource to drop the tracking handles

                // (note: if you don't have any other handles to the assets
                // elsewhere, they will get unloaded after this)
            }

            _ => {
                ready = false;
                // NotLoaded/Loading: not fully ready yet
            }
        }
    }

    if ready {
        println!("ready!");
        println!("{:?}", asset_handler.next_state.state);
        asset_handler.asset_loading.asset_handles = vec![];
        asset_handler
            .state
            .set(asset_handler.next_state.state)
            .unwrap();
        println!("{:?}", asset_handler.next_state.state);
    }
}
