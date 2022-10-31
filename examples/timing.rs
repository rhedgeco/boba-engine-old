use boba_core::{controller_storage::ControllerStorage, *};
use milk_tea_runner::MilkTeaRunner;

#[derive(Debug)]
struct Update;

impl BobaStage for Update {
    fn run(&mut self, controllers: &mut ControllerStorage, resources: &mut BobaResources) {
        controllers.update(self, resources)
    }
}

struct TimeTestController;

impl ControllerStage<Update> for TimeTestController {
    fn update(&mut self, _: &mut Update, resources: &mut BobaResources) {
        println!("FPS: {:?}", 1. / resources.time().delta());
    }
}

register_controller_with_stages!(TimeTestController: Update);

fn main() {
    let mut app = BobaApp::default();
    let controller = BobaController::build(TimeTestController);
    app.stages().add(Update);
    app.controllers().add(controller);
    app.run::<MilkTeaRunner>();
}
