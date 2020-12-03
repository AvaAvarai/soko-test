//! A test at creating a Sokoban style game in Rust.
extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use graphics::character::CharacterCache;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("soko-test", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Error creating window");
    let mut gl = GlGraphics::new(opengl);
    let ref mut glyphs = GlyphCache::new("assets/PressStart2P-Regular.ttf", (), TextureSettings::new()).expect("Error loading font");
   
    let mut player_x = 224.0;
    let mut player_y = 224.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0, 0.0, 1.0, 1.0], g);
                use graphics::Transformed;
                    graphics::Image::new_color([0.0, 0.0, 0.0, 1.0]).draw(
                        glyphs.character(32, '@').unwrap().texture,
                        &c.draw_state,
                        c.transform.trans(player_x, player_y),
                        g
                    );
            });
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => player_y -= 32.0,
                    Button::Keyboard(Key::Down) => player_y += 32.0,
                    Button::Keyboard(Key::Left) => player_x -= 32.0,
                    Button::Keyboard(Key::Right) => player_x += 32.0,
                    _ => (),
                }
            }
        }
    }
} 
