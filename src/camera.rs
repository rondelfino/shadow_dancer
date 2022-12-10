use bevy::core_pipeline::clear_color::ClearColorConfig;

use crate::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    let half_height = WORLD_HEIGHT / 2.0;
    let half_width = WORLD_WIDTH / 2.0;

    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(WORLD_HEIGHT),
            scale: 0.55,
            left: -half_width,
            right: half_width,
            top: half_height,
            bottom: -half_height,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
        },
        ..Default::default()
    };

    commands.spawn(camera_bundle);
}
