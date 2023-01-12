use boba::prelude::*;

struct Stage1;
struct Stage2;
struct Stage3;
struct Stage4;

impl BobaStage for Stage1 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) -> BobaResult {
        println!("Running Stage1");
        Ok(())
    }
}

impl BobaStage for Stage2 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) -> BobaResult {
        println!("Running Stage2");
        Ok(())
    }
}

impl BobaStage for Stage3 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) -> BobaResult {
        println!("Running Stage3");
        Ok(())
    }
}

impl BobaStage for Stage4 {
    type Data = ();

    fn run(&mut self, _: &mut PearlRegistry, _: &mut BobaResources) -> BobaResult {
        println!("Running Stage4");
        Ok(())
    }
}

fn main() {
    let mut app = Bobarista::<TaroGraphicsAdapter>::default();

    app.startup_stages.insert(Stage3);
    app.startup_stages.prepend(Stage2);
    app.startup_stages.append(Stage4);
    app.startup_stages.insert(Stage2);
    app.startup_stages.prepend(Stage1);

    app.run().unwrap();
}
