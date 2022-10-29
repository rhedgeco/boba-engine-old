use boba_core::{controller_storage::ControllerStorage, *};

#[derive(Debug)]
struct TestStage1;

impl BobaStage for TestStage1 {
    fn run(&mut self, controllers: &mut ControllerStorage, resources: &mut BobaResources) {
        controllers.update(self, resources)
    }
}

#[derive(Debug)]
struct TestStage2;

impl BobaStage for TestStage2 {
    fn run(&mut self, controllers: &mut ControllerStorage, resources: &mut BobaResources) {
        controllers.update(self, resources)
    }
}

struct TestController {
    _data: u64,
}

impl ControllerStage<TestStage1> for TestController {
    fn update(&mut self, data: &mut TestStage1, _: &mut BobaResources) {
        println!("Update with {:?} data:{:?} and increment", data, self._data);
        self._data += 1;
    }
}

impl ControllerStage<TestStage2> for TestController {
    fn update(&mut self, data: &mut TestStage2, _: &mut BobaResources) {
        println!("Update with {:?} data:{:?}", data, self._data);
    }
}

register_controller_stages!(TestController: TestStage1, TestStage2);

fn main() {
    let mut app = BobaApp::default();
    let test_stage1 = TestStage1;
    let test_stage2 = TestStage2;
    let controller = BobaController::new(TestController { _data: 5 });

    app.stages().add(test_stage1);
    app.stages().add(test_stage2);
    app.controllers().add(controller);
    app.update();
    app.update();
    app.update();
}
