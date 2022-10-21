// use std::time::Instant;

use boba_input::*;
use raster::Color;

pub struct BobaApp {}

impl Default for BobaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl BobaApp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self) {}

    pub fn keyboard_input(&mut self, state: KeyState, key: Option<KeyCode>, scancode: u32) {
        println!(
            "Key {:?} {:?} -> {:?} on thread {:?}",
            scancode,
            state,
            key,
            std::thread::current().id()
        )
    }

    pub fn render(&mut self, renderer: &mut dyn BobaRenderer) {
        renderer.render_color(Color::white());
    }
}

pub trait BobaRunner {
    fn run(self, app: BobaApp);
}

pub trait BobaRenderer {
    fn render_color(&mut self, color: Color);
}
