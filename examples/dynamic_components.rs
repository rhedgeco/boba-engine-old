use boba_component::{register_updaters, Component, DataUpdate, RegisteredUpdater};
use std::{any::TypeId, mem};

#[derive(Debug)]
struct TestItem1;

#[derive(Debug)]
struct TestItem2;

struct TestData;

impl DataUpdate<TestItem1> for TestData {
    fn update(&self, item: TestItem1) {
        println!("Update 1 with {:?}", item);
    }
}

impl DataUpdate<TestItem2> for TestData {
    fn update_mut(&mut self, item: TestItem2) {
        println!("Update mut 2 with {:?}", item);
    }
}

register_updaters!(TestData: TestItem1, TestItem2);

fn main() {
    let mut component = Component::new(TestData);
    component.update(TestItem1);
    component.update_mut(TestItem2);
}
