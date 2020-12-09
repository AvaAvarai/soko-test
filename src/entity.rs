pub struct Entity {
    x: i32,
    y: i32,
    color: [f32; 4]
}

impl Entity {
    pub fn new(x: i32, y: i32, color: [f32;4]) -> Self {
        Entity {x, y, color}
    }

    pub fn move_self(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn get_coords(&self) ->  (i32, i32) {
        (self.x, self.y)
    }

    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
}
