use bevy::prelude::*;
use bevy::window::PresentMode;

mod app;
use app::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AppPlugin)
        .run();
}
