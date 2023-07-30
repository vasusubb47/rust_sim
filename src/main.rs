use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

mod app;
use app::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation".into(),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AppPlugin)
        .run();
}
