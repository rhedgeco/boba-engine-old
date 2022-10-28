use boba_core::*;

#[derive(Debug)]
struct TestItem1;

#[derive(Debug)]
struct TestItem2;

#[derive(Debug)]
struct TestItem3;

struct TestData {
    _data: u64,
}

impl ControllerStage<TestItem1> for TestData {
    fn update(&mut self, data: &mut TestItem1, _: &mut BobaResources) {
        println!("Update with {:?} data:{:?} and increment", data, self._data);
        self._data += 1;
    }
}

impl ControllerStage<TestItem2> for TestData {
    fn update(&mut self, data: &mut TestItem2, _: &mut BobaResources) {
        println!("Update with {:?} data:{:?}", data, self._data);
    }
}

register_stages!(TestData: TestItem1, TestItem2);

fn main() {
    let mut app = BobaApp::default();
    let test1 = TestData { _data: 5 };
    let cont1 = BobaController::new(test1);
    app.add_controller(cont1.clone());
    app.update(&mut TestItem1);
    app.update(&mut TestItem2);
    app.update(&mut TestItem3);
    app.update(&mut TestItem1);
    app.update(&mut TestItem2);
    app.update(&mut TestItem3);
}
