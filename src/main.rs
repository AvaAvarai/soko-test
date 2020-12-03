//! A test at creating a Sokoban style game in Rust.
extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::RenderEvent;
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("soko-test", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Error creating window");
    let mut gl = GlGraphics::new(opengl);
   
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            graphics::clear([0.0, 0.0, 1.0, 1.0], &mut gl);
            // Main Loop
        }
    }
}
