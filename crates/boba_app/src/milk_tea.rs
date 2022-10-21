use crate::{BobaApp, BobaEvent};

pub struct MilkTea {}

impl MilkTea {
    pub fn new() -> Self {
        MilkTea {}
    }

    fn update(&mut self) {
        println!("update");
    }
}

impl BobaApp for MilkTea {
    fn handle_event(&mut self, event: crate::BobaEvent) {
        match event {
            BobaEvent::Update => {
                self.update();
            }
        }
    }
}
