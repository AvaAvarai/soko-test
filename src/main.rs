//! An attempt at prototyping a turn-based Sokoban-styled puzzle game with Rust and the Piston game engine.
extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod board;
mod level;

const TILE_SIZE: f64 = 32.0;
const TILE_RECT: [f64; 4] = [0.0, 0.0, TILE_SIZE, TILE_SIZE];
const WINDOW_SIZE: (u32, u32) = (TILE_SIZE as u32 * 11, 480);

// Colors
type Color = [f32; 4];
const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];


fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("soko-test", [WINDOW_SIZE.0, WINDOW_SIZE.1])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("PressStart2P-Regular.ttf")).unwrap();

    let mut level_num: i32 = 1;
    let mut current_level: board::Board = level::load_level(level_num);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, d| {
            clear(BLUE, g);
            let current_board = current_level.get_board();
            
            for goal in current_level.get_goals() {
                let trans = c.transform.trans(
                    goal.0 as f64 * TILE_SIZE,
                    goal.1 as f64 * TILE_SIZE
                );
                rectangle(RED, TILE_RECT, trans, g);
            }            

            for (i, row) in current_board.iter().enumerate() {
                for (q, col) in row.iter().enumerate() {
                    let trans = c.transform.trans(
                        i as f64 * TILE_SIZE,
                        q as f64 * TILE_SIZE
                    );
                    match col {
                        'P' => rectangle(WHITE, TILE_RECT, trans, g), // Player
                        'B' => rectangle(GREEN, TILE_RECT, trans, g), // Moveable-Box
                        '#' => rectangle(BLACK, TILE_RECT, trans, g), // Wall
                        _ => (),
                    }
                }
            }

            let transform = c.transform.trans(10.0, 400.0);
            text::Text::new_color(GREEN, 18).draw(
                &format!("Moves: {}", current_level.get_moves_made().to_string()),
                &mut glyphs, &c.draw_state, transform, g).unwrap();

            let transform = c.transform.trans(10.0, 440.0);
            text::Text::new_color(GREEN, 18).draw(
                &format!("Goal: {}", current_level.get_moves_goal().to_string()),
                &mut glyphs, &c.draw_state, transform, g).unwrap();

            if *current_level.get_level_solved() {
                let transform = c.transform.trans(50.0, 60.0);
                text::Text::new_color(GREEN, 18).draw(
                "Level Done!",
                &mut glyphs, &c.draw_state, transform, g).unwrap();
                level_num += 1;
                current_level = level::load_level(level_num);
            }

            glyphs.factory.encoder.flush(d);
        });

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => current_level.move_player(0, -1),
                    Button::Keyboard(Key::Down) => current_level.move_player(0, 1),
                    Button::Keyboard(Key::Left) => current_level.move_player(-1, 0),
                    Button::Keyboard(Key::Right) => current_level.move_player(1, 0),
                    Button::Keyboard(Key::R) => current_level = level::load_level(level_num),
                    _ => (),
                }
            }
        }
    }
}
