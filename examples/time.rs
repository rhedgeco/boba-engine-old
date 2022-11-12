use std::time::Instant;

use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{prelude::*, TaroRenderer};

struct Time {
    instant: Instant,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}

impl BobaController for Time {}

impl BobaUpdate<MainBobaUpdate> for Time {
    fn update(&mut self, _: &(), _: &mut BobaResources) {
        println!("FPS: {:?}", 1. / self.instant.elapsed().as_secs_f32());
        self.instant = Instant::now();
    }
}

fn main() {
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    app.stages()
        .add_controller(BobaContainer::build(Time::default()));
    app.resources().add(TaroRenderer::default());
    MilkTeaRunner::run(app).unwrap();
}
