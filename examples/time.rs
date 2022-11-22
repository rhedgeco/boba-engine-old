use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{prelude::*, TaroRenderer};

struct Time;

impl PearlRegister for Time {
    fn register(pearl: Pearl<Self>, storage: &mut storage::StageRunners) {
        storage.add(pearl);
    }
}

impl PearlStage<MainBobaUpdate> for Time {
    fn update(&mut self, delta: &f32, _: &mut BobaResources) -> PearlResult {
        println!("FPS: {:?}", 1. / delta);
        Ok(())
    }
}

fn main() {
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    app.stages.add_pearl(Time.into_pearl());
    app.resources.add(TaroRenderer::default());
    MilkTeaRunner::run(app).unwrap();
}
