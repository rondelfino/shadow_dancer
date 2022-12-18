use crate::{assets::GameAssets, prelude::*, run_after_bonus_stage_intro};

#[derive(Bundle)]
pub struct RoofBundle {
    roof: Roof,
    dimensions: Dimensions,
    sprite_bundle: SpriteBundle,
}

impl RoofBundle {
    pub fn left_roof(game_assets: &Res<GameAssets>, y_pos: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(174.0, 224.0));

        let sprite_bundle = SpriteBundle {
            texture: game_assets.left_roof.clone(),
            transform: Transform {
                translation: Vec3::new(LEFT_WALL + (dimensions.0.x / 2.0) - 48.0, y_pos, 1.0),
                ..default()
            },
            ..default()
        };
        RoofBundle {
            roof: Roof,
            dimensions,
            sprite_bundle,
        }
    }

    pub fn right_roof(game_assets: &Res<GameAssets>, y_pos: f32) -> Self {
        let dimensions = Dimensions(Vec2::new(48.0, 224.0));

        let sprite_bundle = SpriteBundle {
            texture: game_assets.right_roof.clone(),
            transform: Transform {
                translation: Vec3::new(RIGHT_WALL + dimensions.0.x / 2.0, y_pos, 1.0),
                ..default()
            },
            ..default()
        };

        RoofBundle {
            roof: Roof,
            dimensions,
            sprite_bundle,
        }
    }
}

pub struct RoofPlugin;
impl Plugin for RoofPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_system(build_towers),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_run_criteria(run_after_bonus_stage_intro)
                .with_system(roof_animator),
        );
    }
}

pub fn build_towers(mut commands: Commands, game_assets: Res<GameAssets>) {
    let left_wall_roof = RoofBundle::left_roof(&game_assets, -100.0);

    let left_roof_height = left_wall_roof.dimensions.0.y;

    commands.spawn(left_wall_roof);

    let right_wall_roof = RoofBundle::right_roof(&game_assets, -100.0);

    let right_roof_height = right_wall_roof.dimensions.0.y;
    commands.spawn(right_wall_roof);

    for i in -4..=0 {
        commands.spawn(WallBundle::left_wall(&game_assets, i, left_roof_height));
        commands.spawn(WallBundle::right_wall(&game_assets, i, right_roof_height));
    }
}

pub fn roof_animator(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Dimensions), With<Roof>>,
    time: Res<Time>,
) {
    for (roofs, mut roof_transform, roof_dimensions) in query.iter_mut() {
        roof_transform.translation.y += FALLING_SPEED * time.delta_seconds();

        if roof_transform.translation.y > 3.0 * roof_dimensions.0.y {
            commands.entity(roofs).insert(MarkDespawn);
        }
    }
}
