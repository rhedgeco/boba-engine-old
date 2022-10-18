use std::time::Instant;

use raster::Color;

pub struct BobaApp {
    instant: Instant,
}

impl BobaApp {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        println!("Update App {:?}", 1. / self.instant.elapsed().as_secs_f64());
        self.instant = Instant::now();
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
