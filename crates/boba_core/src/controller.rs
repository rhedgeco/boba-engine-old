use uuid::Uuid;

use crate::{BobaResources, BobaStage};

#[derive(Debug, Clone)]
pub struct BobaController<T: ControllerData> {
    uuid: Uuid,
    data: T,
}

impl<T: ControllerData> BobaController<T> {
    pub fn build(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            data,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

pub trait ControllerData {}

pub trait ControllerStage<Stage: 'static + BobaStage>: ControllerData {
    fn update<'a>(&'a mut self, data: &mut Stage::StageData<'a>, resources: &mut BobaResources);
}
