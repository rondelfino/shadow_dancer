use bevy::core_pipeline::clear_color::ClearColorConfig;

use crate::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(Self::camera_setup));
    }
}

impl CameraPlugin {
    pub fn camera_setup(mut commands: Commands) {
        let camera_bundle = Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.0,
                scaling_mode: ScalingMode::FixedVertical(WORLD_HEIGHT),
                scale: CAMERA_SCALE,
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            },
            ..Default::default()
        };

        commands.spawn(camera_bundle);
    }
}
