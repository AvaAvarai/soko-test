//! A test at creating a Sokoban style game in Rust.
extern crate piston_window;

use piston_window::*;

mod entity;

const TILE_SIZE :f64 = 32.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("soko-test", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut player: entity::Entity = entity::Entity::new(2, 2, [1.0; 4]);
    let mut moveable_box: entity::Entity = entity::Entity::new(6, 2, [0.0, 1.0, 0.0, 1.0]);

    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g);
            
            let (player_x, player_y) = player.get_coords();

            // Draw Player
            let trans = c.transform.trans(
                *player_x as f64 * TILE_SIZE,
                *player_y as f64 * TILE_SIZE
            );
            rectangle(*player.get_color(),
                [0.0, 0.0, TILE_SIZE, TILE_SIZE],
                trans,
                g);

            let (moveable_box_x, moveable_box_y) = moveable_box.get_coords();

            // Draw Moveable_Box
            let trans = c.transform.trans(
                *moveable_box_x as f64 * TILE_SIZE,
                *moveable_box_y as f64 * TILE_SIZE
            );
            rectangle(*moveable_box.get_color(),
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
