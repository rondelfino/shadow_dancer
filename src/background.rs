use crate::{assets::GameAssets, prelude::*};

#[derive(Bundle)]
pub struct BackgroundBundle {
    background: Background,
    dimensions: Dimensions,
    sprite_bundle: SpriteBundle,
}

impl BackgroundBundle {
    pub fn new(game_assets: Res<GameAssets>) -> Self {
        let dimensions = Dimensions(Vec2::new(1248.0, 1667.0));

        let scale = WORLD_WIDTH / (dimensions.0.x - 207.0);
        let sprite_bundle = SpriteBundle {
            texture: game_assets.background.clone(),
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    -((dimensions.0.y * scale) / 2.0) + (WORLD_HEIGHT / 4.0) + 55.0,
                    0.0,
                ),
                scale: Vec3::new(scale, scale, 0.0),
                ..default()
            },
            ..default()
        };
        BackgroundBundle {
            background: Background,
            dimensions,
            sprite_bundle,
        }
    }
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_background))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(background_animator),
            );
    }
}

pub fn spawn_background(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn_empty()
        .insert(BackgroundBundle::new(game_assets));
}

pub fn background_animator(mut query: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    for mut background_transform in query.iter_mut() {
        background_transform.translation.y += (FALLING_SPEED / 100.0) * time.delta_seconds();
    }
}
