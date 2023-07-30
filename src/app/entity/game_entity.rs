use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Debug)]
pub struct GameEntity {
    pub uuid: Uuid,
}

impl GameEntity {
    pub fn new() -> Self {
        println!("Spawning a new game entity");
        GameEntity {
            uuid: Uuid::new_v4(),
        }
    }
}

impl Default for GameEntity {
    fn default() -> Self {
        println!("Spawning a default game entity");
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}
