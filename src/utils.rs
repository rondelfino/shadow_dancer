use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum GameState {
    Initial,
    Splash,
    Transition,
    TitleScreen,
    MainMenu,
    InGame,
    Paused,
    Loading,
    LoadWorld,
    EndStage,
}

pub struct Bounds {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

pub fn calculate_bounds(transform: &Transform, size: Option<Vec2>) -> Bounds {
    let left_bound = transform.translation.x - size.unwrap_or_default().x / 2.0;
    let right_bound = transform.translation.x + size.unwrap_or_default().x / 2.0;
    let top_bound = transform.translation.y + size.unwrap_or_default().y / 2.0;
    let bottom_bound = transform.translation.y - size.unwrap_or_default().y / 2.0;

    Bounds {
        top: top_bound,
        right: right_bound,
        bottom: bottom_bound,
        left: left_bound,
    }
}

pub fn despawner<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
