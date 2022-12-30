use boba::prelude::*;
use milk_tea::stages::MilkTeaUpdate;

pub struct FpsPrinter;

impl RegisterStages for FpsPrinter {
    fn register(pearl: &Pearl<Self>, stages: &mut impl StageRegistrar) {
        stages.add(pearl.clone());
    }
}

impl PearlStage<MilkTeaUpdate> for FpsPrinter {
    fn update(&mut self, delta: &f32, _: &mut BobaResources) -> BobaResult {
        println!("FPS: {:.0}", 1. / delta);
        Ok(())
    }
}

fn main() {
    let mut app = Bobarista::<TaroMilkTea>::default();
    app.registry.add(&FpsPrinter.wrap_pearl());
    app.run().unwrap();
}
