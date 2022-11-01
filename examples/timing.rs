use boba_core::*;
use milk_tea_runner::{stages::MilkTeaUpdate, *};

struct TimeTestController;

impl ControllerStage<MilkTeaUpdate> for TimeTestController {
    fn update(&mut self, _: &mut MilkTeaUpdate, resources: &mut BobaResources) {
        let delta = resources.time().delta();
        println!("FPS: {:?}", 1. / delta);
    }
}

register_controller_with_stages!(TimeTestController: MilkTeaUpdate);

fn main() {
    let mut app = BobaApp::new(MilkTeaRunner::default());
    app.controllers()
        .add(BobaController::build(TimeTestController));
    app.run();
}
