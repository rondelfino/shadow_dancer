use crate::{asset_loading::AssetHandler, assets::GameAssets, prelude::*};

pub struct GameScriptPlugin;
impl Plugin for GameScriptPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScriptState>()
            .add_system_set(SystemSet::on_update(GameState::LoadWorld).with_system(load_state));
    }
}

#[derive(Debug)]
pub enum GameScript {
    BonusStage,
}

#[derive(Resource)]
pub struct GameScriptState {
    pub current: GameScript,
}

impl Default for GameScriptState {
    fn default() -> Self {
        GameScriptState {
            current: GameScript::BonusStage,
        }
    }
}

// impl GameScriptState {
//     pub fn next(&mut self) {
//         println!("Moving from {:?}", self.current);
//         // self.current = match self.current {};
//         println!("to {:?}", self.current);
//     }
// }

fn load_state(
    mut assets_handler: AssetHandler,
    mut game_assets: ResMut<GameAssets>,

    game_script_state: Res<GameScriptState>,
) {
    println!("Loading state {:?}", game_script_state.current);
    match game_script_state.current {
        GameScript::BonusStage => {
            assets_handler.load(GameState::InGame, &mut game_assets)
        }
        _ => (),
    }
}
