//! A test at creating a Sokoban style game in Rust.
extern crate glutin_window;
extern crate piston;

use glutin_window::GlutinWindow;
use piston::WindowSettings;
use piston::event_loop::{EventSettings, Events};

fn main() {
    let settings = WindowSettings::new("soko-test", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Error creating window");

    let mut events = Events::new(EventSettings::new());
    while let Some(_) = events.next(&mut window) {
        // Main Loop
    }

}
