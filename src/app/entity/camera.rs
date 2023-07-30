use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use super::game_entity::GameEntity;

#[derive(Component, Debug)]
pub struct Camera;

impl Camera {
    fn init(mut commands: Commands) {
        commands.spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    // no "background color", we need to see the main camera's output
                    // rgb(0, 231, 255)
                    clear_color: ClearColorConfig::Custom(Color::rgb_u8(0, 231, 255)),
                    ..default()
                },
                ..default()
            },
            Camera,
            GameEntity::default(),
        ));
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Camera::init);
    }
}
