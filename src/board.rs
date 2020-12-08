use super::entity;

pub struct Board {
    player: entity::Entity,     // Controllable Entity
    boxes: Vec<entity::Entity>, // Interactable Entity
    goals: Vec<entity::Entity>, // Static Game Object
    walls: Vec<entity::Entity>  // Static Game Object
}

impl Board {
    pub fn new(player: entity::Entity, boxes: Vec<entity::Entity>, goals: Vec<entity::Entity>, walls: Vec<entity::Entity>) -> Self {
        Board {player, boxes, goals, walls}
    }

    pub fn get_player(&self) -> &entity::Entity {
        &self.player
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        self.player.move_self(dx, dy);
    }

    pub fn get_boxes(&self) -> &Vec<entity::Entity> {
        &self.boxes
    }
}
