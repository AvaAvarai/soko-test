//! A test at creating a Sokoban style game in Rust.
extern crate piston_window;

use piston_window::*;

mod entity;
mod board;

const TILE_SIZE :f64 = 32.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("soko-test", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Test Level
    let player: entity::Entity = entity::Entity::new(2, 2, [1.0; 4]);
    let first_box: entity::Entity = entity::Entity::new(6, 2, [0.0, 1.0, 0.0, 1.0]);
    let first_goal: entity::Entity = entity::Entity::new(6, 4, [1.0, 0.0, 0.0, 1.0]);
    let mut current_level: board::Board = board::Board::new(player, vec![first_box], vec![first_goal], vec![]); 

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g);

            // Draw Player     
            let (player_x, player_y) = current_level.get_player().get_coords();
            
            let trans = c.transform.trans(
                *player_x as f64 * TILE_SIZE,
                *player_y as f64 * TILE_SIZE
            );
            rectangle(*current_level.get_player().get_color(),
                [0.0, 0.0, TILE_SIZE, TILE_SIZE],
                trans,
                g);

            // Draw Boxes
            for moveable_box in current_level.get_boxes() {
                let (moveable_box_x, moveable_box_y) = moveable_box.get_coords();
                let trans = c.transform.trans(
                    *moveable_box_x as f64 * TILE_SIZE,
                    *moveable_box_y as f64 * TILE_SIZE
                );
                rectangle(*moveable_box.get_color(),
                    [0.0, 0.0, TILE_SIZE, TILE_SIZE],
                    trans,
                    g);
            }
        });

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => current_level.move_player(0, -1),
                    Button::Keyboard(Key::Down) => current_level.move_player(0, 1),
                    Button::Keyboard(Key::Left) => current_level.move_player(-1, 0),
                    Button::Keyboard(Key::Right) => current_level.move_player(1, 0),
                    _ => (),
                }
            }
        }
    }
}
