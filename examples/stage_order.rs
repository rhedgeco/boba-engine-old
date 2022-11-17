use boba_core::*;
use milk_tea_runner::*;

struct TestStage1;
struct TestStage2;
struct TestStage3;
struct TestStage4;

impl BobaStage for TestStage1 {
    type StageData = ();

    fn run(&mut self, _: &mut storage::PearlStorage<Self>, _: &mut BobaResources)
    where
        Self: 'static,
    {
        println!("Test Stage 1");
    }
}

impl BobaStage for TestStage2 {
    type StageData = ();

    fn run(&mut self, _: &mut storage::PearlStorage<Self>, _: &mut BobaResources)
    where
        Self: 'static,
    {
        println!("Test Stage 2");
    }
}

impl BobaStage for TestStage3 {
    type StageData = ();

    fn run(&mut self, _: &mut storage::PearlStorage<Self>, _: &mut BobaResources)
    where
        Self: 'static,
    {
        println!("Test Stage 3");
    }
}

impl BobaStage for TestStage4 {
    type StageData = ();

    fn run(&mut self, _: &mut storage::PearlStorage<Self>, _: &mut BobaResources)
    where
        Self: 'static,
    {
        println!("Test Stage 4");
    }
}

fn main() {
    let mut app = BobaApp::default();

    // insert second stage
    app.stages.insert(TestStage2);

    // fourth stage is inserted out of order
    app.stages.insert(TestStage4);

    // use insert_after to keep correct order for third stage
    // possible errors are:
    //     IdenticalTypes -> Cannot insert a stage after itself
    //     StageNotFound -> Cannot insert after a stage that doesnt exist
    app.stages
        .insert_after::<TestStage2, TestStage3>(TestStage3)
        .unwrap();

    // use prepend to insert first stage at the beginning
    app.stages.prepend(TestStage1);

    // when run, you will see each stage print in order
    MilkTeaRunner::run(app).unwrap();
}
