use boba_core::{BobaResources, BobaStage, PearlRegistry};
use milk_tea::MilkTeaApp;

struct Stage1;
struct Stage2;
struct Stage3;
struct Stage4;

impl BobaStage for Stage1 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) {
        println!("Running Stage1");
    }
}

impl BobaStage for Stage2 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) {
        println!("Running Stage2");
    }
}

impl BobaStage for Stage3 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) {
        println!("Running Stage3");
    }
}

impl BobaStage for Stage4 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) {
        println!("Running Stage4");
    }
}

fn main() {
    let mut app = MilkTeaApp::default();

    app.startup_stages.insert(Stage3);
    app.startup_stages.prepend(Stage2);
    app.startup_stages.append(Stage4);
    app.startup_stages.insert(Stage2);
    app.startup_stages.prepend(Stage1);

    app.run().unwrap();
}
