use boba_objects::{register_updaters, DataUpdate, RegisteredUpdater};

#[derive(Debug)]
struct TestItem1;

#[derive(Debug)]
struct TestItem2;

struct TestData {
    _data: u64,
}

impl DataUpdate<TestItem1> for TestData {
    fn update(&mut self, item: &TestItem1) {
        println!("Update 1 with {:?}", item);
    }
}

impl DataUpdate<TestItem2> for TestData {
    fn update(&mut self, item: &TestItem2) {
        println!("Update 2 with {:?}", item);
    }
}

register_updaters!(TestData: TestItem1, TestItem2);

fn main() {}
