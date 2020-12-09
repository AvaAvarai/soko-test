//! A test at creating a Sokoban style game in Rust.
extern crate piston_window;

use piston_window::*;

mod entity;
mod board;

const TILE_SIZE: f64 = 32.0;
const TILE_RECT: [f64; 4] = [0.0, 0.0, TILE_SIZE, TILE_SIZE];

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
    let mut test_walls = vec![];
    for num in 0..11 {
        let test_wall: entity::Entity = entity::Entity::new(0, num, [0.0, 0.0, 0.0, 1.0]);
        let test_wall2: entity::Entity = entity::Entity::new(num, 0, [0.0, 0.0, 0.0, 1.0]);
        let test_wall3: entity::Entity = entity::Entity::new(num, 10, [0.0, 0.0, 0.0, 1.0]);
        let test_wall4: entity::Entity = entity::Entity::new(10, num, [0.0, 0.0, 0.0, 1.0]);
        test_walls.push(test_wall);
        test_walls.push(test_wall2);
        test_walls.push(test_wall3);
        test_walls.push(test_wall4);
    }
    let mut current_level: board::Board = board::Board::new(player, vec![first_box], vec![first_goal], test_walls); 

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g);

            // Draw Player     
            let (player_x, player_y) = current_level.get_player().get_coords();
            
            let trans = c.transform.trans(
                player_x as f64 * TILE_SIZE,
                player_y as f64 * TILE_SIZE
            );
            rectangle(current_level.get_player().get_color(),
                TILE_RECT,
                trans,
                g);

            // Draw Boxes
            for moveable_box in current_level.get_boxes() {
                let (moveable_box_x, moveable_box_y) = moveable_box.get_coords();
                let trans = c.transform.trans(
                    moveable_box_x as f64 * TILE_SIZE,
                    moveable_box_y as f64 * TILE_SIZE
                );
                rectangle(moveable_box.get_color(),
                    TILE_RECT,
                    trans,
                    g);
            }

            // Draw Goals
            for game_goal in current_level.get_goals() {
                let (game_goal_x, game_goal_y) = game_goal.get_coords();
                let trans = c.transform.trans(
                    game_goal_x as f64 * TILE_SIZE,
                    game_goal_y as f64 * TILE_SIZE
                );
                rectangle(game_goal.get_color(),
                    TILE_RECT,
                    trans,
                    g)
            }

            // Draw Walls
            for board_wall in current_level.get_walls() {
                let (board_wall_x, board_wall_y) = board_wall.get_coords();
                let trans = c.transform.trans(
                    board_wall_x as f64 * TILE_SIZE,
                    board_wall_y as f64 * TILE_SIZE
                );
                rectangle(board_wall.get_color(),
                    TILE_RECT,
                    trans,
                    g)
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
