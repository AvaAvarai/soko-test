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
        let mut need_recheck = false;

        let mut test_coords = self.get_player().get_coords();
        test_coords.0 += dx;
        test_coords.1 += dy;

        // Player-Wall Collision
        for wall in self.get_walls() {
            let wall_coords = wall.get_coords();
            if test_coords == wall_coords {
                return
            }
        }

        // Player-Box Movement
        let test_boxes = &self.boxes.clone();
        let test_walls = &self.walls.clone();
        for moveable_box in self.boxes.iter_mut() {
            let box_coords = moveable_box.get_coords();
            if test_coords == box_coords {
                let mut box_test_coords = moveable_box.get_coords();
                box_test_coords.0 += dx;
                box_test_coords.1 += dy;
                for wall in self.walls.iter() {
                    let wall_coords = wall.get_coords();
                    if box_test_coords == wall_coords {
                        return
                    }
                }
                for moveable_box in test_boxes {
                    if moveable_box.get_x() == box_test_coords.0 && moveable_box.get_y() == box_test_coords.1 {
                        return
                    }
                }
                for wall in test_walls {
                    if wall.get_x() == box_test_coords.0 && wall.get_y() == box_test_coords.1 {
                        return
                    }
                }
                moveable_box.move_self(dx, dy);
                
                
                for goal in self.goals.iter() {
                    let goal_coords = goal.get_coords();
                    if box_test_coords == goal_coords {
                        need_recheck = true;
                    }
                }
            }
        }

        if need_recheck {
            if self.recheck_boxes() {
                // Trigger Next Level
                println!("Level Complete.");
            }
        }

        // Player-Empty Movement
        self.player.move_self(dx, dy);
    }

    // Possible replacement for cloning on lines 35-36.
    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        for moveable_box in &self.boxes {
            if moveable_box.get_x() == x && moveable_box.get_y() == y {
                return true
            }
        }
        for wall in self.get_walls() {
            if wall.get_x() == x && wall.get_y() == y {
                return true
            }
        }
        return false
    }

    pub fn recheck_boxes(&self) -> bool {
        let mut num = 0;
        for goal in self.goals.iter() {
            for moveable_box in self.boxes.iter() {
                if goal.get_coords() == moveable_box.get_coords() {
                    num+=1;
                }
            }
        }
        if num == self.boxes.len() {
            return true
        }
        false
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
