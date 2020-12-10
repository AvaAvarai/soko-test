//! An attempt at prototyping a turn-based Sokoban-styled puzzle game with Rust and the Piston game engine.
extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod board;

const TILE_SIZE: f64 = 32.0;
const TILE_RECT: [f64; 4] = [0.0, 0.0, TILE_SIZE, TILE_SIZE];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("soko-test", [480, 480])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Test Level
    let mut test_level: Vec< Vec<char>> = vec![vec!['.'; 11]; 11];
    
    test_level[2][2] = 'P';
    test_level[6][2] = 'B';
    test_level[3][2] = 'B';

    for num_a in 0..11 as usize {
        for num_b in 0..11 as usize {
            if num_a == 0 || num_a == 10 || num_b == 0 || num_b == 10 {
                test_level[num_a][num_b] = '#';
            }
        }   
    }

    let mut current_level: board::Board = board::Board::new(test_level, (2, 2), vec![(4, 4), (6, 4)], 0);
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let mut glyphs = window.load_font(assets.join("PressStart2P-Regular.ttf")).unwrap();
  
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, d| {
            clear([0.0, 0.0, 1.0, 1.0], g);

            let current_board = current_level.get_board();
            for goal in current_level.get_goals() {
                let trans = c.transform.trans(
                    goal.0 as f64 * TILE_SIZE,
                    goal.1 as f64 * TILE_SIZE
                );
                rectangle([1.0, 0.0, 0.0, 1.0], TILE_RECT, trans, g);
            }            

            for (i, row) in current_board.iter().enumerate() {
                for (q, col) in row.iter().enumerate() {
                    let trans = c.transform.trans(
                        i as f64 * TILE_SIZE,
                        q as f64 * TILE_SIZE
                    );
                    let color: [f32; 4];
                    match col {
                        'P' => color = [1.0; 4], // Player
                        'B' => color = [0.0, 1.0, 0.0, 1.0], // Moveable-Box
                        '#' => color = [0.0, 0.0, 0.0, 1.0], // Wall
                        _ => color = [0.0; 4]
                    }
                    rectangle(color, TILE_RECT, trans, g);
                }
            }

            let transform = c.transform.trans(10.0, 400.0);

            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 18).draw(
                &format!("Moves Made: {}", current_level.get_moves_made().to_string()),
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            glyphs.factory.encoder.flush(d);
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
