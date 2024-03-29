use boba::prelude::*;
use taro_milk_tea::TaroGraphicsAdapter;

pub struct FpsPrinter;

register_pearl_stages!(FpsPrinter: BobaUpdate);

impl PearlStage<BobaUpdate> for FpsPrinter {
    fn update(_pearl: &Pearl<Self>, delta: &f32, _: &mut BobaResources) -> BobaResult {
        println!("FPS: {:.0}", 1. / delta);
        Ok(())
    }
}

fn main() {
    let mut app = MilkTeaApp::default();
    app.registry.add(Pearl::wrap(FpsPrinter));
    app.run::<TaroGraphicsAdapter>().unwrap();
}
