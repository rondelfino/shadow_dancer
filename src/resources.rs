use crate::prelude::*;

#[derive(Resource, Clone)]
pub struct EnemyCount(pub u32);

#[derive(Default, Resource, PartialEq, Eq, Debug)]
pub enum PauseEvent {
    Paused,
    #[default]
    Unpaused,
}
