use bevy::{prelude::*, window::PrimaryWindow};

use super::{game_entity::GameEntity, moveable_game_entity::MoveableGameEntity};

#[derive(Debug, Component)]
pub struct Player {
    pub size: Vec2,
}

fn spawn_player(
    mut command: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    // let x = 0.0;
    // let y = 0.0;

    command.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load("sprites/ship_sidesC.png"),
            ..default()
        },
        GameEntity {
            position: Vec3::new(x, y, 0.0),
            ..default()
        },
        MoveableGameEntity::new(500.0),
        Player {
            size: Vec2::new(128.0, 128.0),
        },
    ));
    println!("Player initialized");
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut GameEntity, &MoveableGameEntity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut game_entity, moveable_game_entity)) = player.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        moveable_game_entity.update_entity(direction, &mut game_entity, time.delta_seconds());
    }
}

fn update_transform(mut player: Query<(&GameEntity, &mut Transform), With<Player>>) {
    if let Ok((game_entity, mut transform)) = player.get_single_mut() {
        transform.translation = game_entity.position;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_system(player_movement);
        app.add_system(update_transform.after(player_movement));
    }
}
