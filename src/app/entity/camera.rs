use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::PrimaryWindow};

use super::game_entity::GameEntity;

#[derive(Component, Debug)]
pub struct Camera;

impl Camera {
    fn init(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
        let window = window_query.get_single().unwrap();
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
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
