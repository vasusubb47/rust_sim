use bevy::prelude::*;

use super::game_entity::GameEntity;

#[derive(Debug, Component)]
pub struct MoveableGameEntity {
    pub entity_speed: f32,
}

impl MoveableGameEntity {
    pub fn new(entity_speed: f32) -> Self {
        Self { entity_speed }
    }

    pub fn update_entity(&self, update_dir: Vec3, game_entity: &mut GameEntity, delta_time: f32) {
        game_entity.update_entity_position(update_dir * self.entity_speed * delta_time);
    }
}

impl Default for MoveableGameEntity {
    fn default() -> Self {
        Self { entity_speed: 0.0 }
    }
}
