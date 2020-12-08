//! A test at creating a Sokoban style game in Rust.
extern crate piston_window;

use piston_window::*;

const TILE_SIZE :f64 = 50.0;

struct Entity {
    x: i32,
    y: i32,
    color: [f32;4]
}

impl Entity {
    pub fn new(x: i32, y: i32, color: [f32;4]) -> Self {
        Entity {x, y, color}
    }

    pub fn move_self(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("soko-test", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut player :Entity = Entity::new(2, 2, [1.0;4]);
    let mut moveable_box :Entity = Entity::new(6, 2, [0.0, 1.0, 0.0, 1.0]);
   
    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g);
            
            // Draw Player
            let trans = c.transform.trans(
                player.x as f64 * TILE_SIZE,
                player.y as f64 * TILE_SIZE
            );
            rectangle(player.color,
                [0.0, 0.0, TILE_SIZE, TILE_SIZE],
                trans,
                g);

            // Draw Moveable_Box
            let trans = c.transform.trans(
                moveable_box.x as f64 * TILE_SIZE,
                moveable_box.y as f64 * TILE_SIZE
            );
            rectangle(moveable_box.color,
                [0.0, 0.0, TILE_SIZE, TILE_SIZE],
                trans,
                g);
        });

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => player.move_self(0, -1),
                    Button::Keyboard(Key::Down) => player.move_self(0, 1),
                    Button::Keyboard(Key::Left) => player.move_self(-1, 0),
                    Button::Keyboard(Key::Right) => player.move_self(1, 0),
                    _ => (),
                }
            }
        }
    }
}
