use boba_core::*;

struct TestItem1;

struct TestItem2;

struct TestItem3;

struct TestData {
    _data: u64,
}

impl ControllerStage<TestItem1> for TestData {
    fn update(&mut self, _: &mut TestItem1, _: &mut BobaResources) {
        println!("Update 1 data:{:?}", self._data);
    }
}

impl ControllerStage<TestItem2> for TestData {
    fn update(&mut self, _: &mut TestItem2, _: &mut BobaResources) {
        println!("Update 2 data:{:?}", self._data);
    }
}

register_stages!(TestData: TestItem1, TestItem2);

fn main() {
    let mut world = BobaApp::default();
    let test1 = TestData { _data: 5 };
    let cont1 = BobaController::new(test1);
    world.add_controller(cont1.clone());
    world.update(&mut TestItem1);
    world.update(&mut TestItem2);
    world.update(&mut TestItem3);
}
