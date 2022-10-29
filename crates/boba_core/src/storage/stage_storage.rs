use std::slice::IterMut;

use crate::BobaStage;

#[derive(Default)]
pub struct StageStorage {
    stages: Vec<Box<dyn BobaStage>>,
}

impl StageStorage {
    pub fn add<Stage: 'static + BobaStage>(&mut self, stage: Stage) {
        self.stages.push(Box::new(stage));
    }

    pub fn iter_mut(&mut self) -> IterMut<Box<dyn BobaStage>> {
        self.stages.iter_mut()
    }
}
