use super::entity;

pub struct Board {
    player: entity::Entity,
    boxes: Vec<entity::Entity>,
    goals: Vec<entity::Entity>,
    walls: Vec<entity::Entity>
}

impl Board {
    pub fn new(player: entity::Entity, boxes: Vec<entity::Entity>, goals: Vec<entity::Entity>, walls: Vec<entity::Entity>) -> Self {
        Board {player, boxes, goals, walls}
    }

    pub fn get_player(&self) -> &entity::Entity {
        &self.player
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let (mut test_x, mut test_y) = self.get_player().get_coords();
        test_x += dx;
        test_y += dy;

        for wall in self.get_walls() {
            let (wall_x, wall_y) = wall.get_coords();
            if test_x == wall_x && test_y == wall_y {
                return
            }
        }

        self.player.move_self(dx, dy);
    }

    pub fn get_boxes(&self) -> &Vec<entity::Entity> {
        &self.boxes
    }

    pub fn get_goals(&self) -> &Vec<entity::Entity> {
        &self.goals
    }

    pub fn get_walls(&self) -> &Vec<entity::Entity> {
        &self.walls
    }
}
