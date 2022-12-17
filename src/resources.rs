use crate::prelude::*;

#[derive(Resource)]
pub struct EnemyCount(pub u32);

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);