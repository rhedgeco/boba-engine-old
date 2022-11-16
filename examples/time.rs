use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{prelude::*, TaroRenderer};

struct Time;

impl StageRegister for Time {
    fn register(pearl: Pearl<Self>, storage: &mut storage::StageRunners) {
        storage.add(pearl);
    }
}

impl BobaUpdate<MainBobaUpdate> for Time {
    fn update(&mut self, delta: &f32, _: &mut BobaResources) {
        println!("FPS: {:?}", 1. / delta);
    }
}

fn main() {
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    app.stages.add_pearl(Pearl::wrap(Time));
    app.resources.add(TaroRenderer::default());
    MilkTeaRunner::run(app).unwrap();
}
