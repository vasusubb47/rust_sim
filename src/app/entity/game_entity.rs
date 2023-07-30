use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Debug)]
pub struct GameEntity {
    pub uuid: Uuid,
    // world position
    pub position: Vec3,
}

impl GameEntity {
    pub fn update_entity_position(&mut self, new_pos: Vec3) {
        self.position += new_pos;
    }
}

impl Default for GameEntity {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            position: Vec3::ZERO,
        }
    }
}
