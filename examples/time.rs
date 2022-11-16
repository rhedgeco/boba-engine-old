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
    fn update(delta: &f32, _: &mut Pearl<Self>, _: &mut BobaResources) -> BobaResult {
        println!("FPS: {:?}", 1. / delta);
        Ok(())
    }
}

fn main() {
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    app.stages.add_pearl(Time.pearl());
    app.resources.add(TaroRenderer::default());
    MilkTeaRunner::run(app).unwrap();
}
