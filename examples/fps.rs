use std::time::Instant;

use boba_core::BobaStage;
use milk_tea::MilkTeaApp;
use taro_renderer::adapters::TaroMilkTea;

struct FpsStage {
    instant: Instant,
}

impl Default for FpsStage {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}

impl BobaStage for FpsStage {
    type Data = ();

    fn run(&mut self, _: &mut boba_core::PearlRegistry, _: &mut boba_core::BobaResources) {
        let fps = 1. / self.instant.elapsed().as_secs_f64();
        self.instant = Instant::now();
        println!("FPS: {fps:.0}");
    }
}

fn main() {
    let mut app = MilkTeaApp::<TaroMilkTea>::default();
    app.main_stages.insert(FpsStage::default());
    app.run().unwrap();
}
