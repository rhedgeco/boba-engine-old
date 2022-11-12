use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{BobaResources, BobaStage};

pub struct BobaController<T: ControllerData> {
    uuid: Uuid,
    data: Rc<RefCell<T>>,
}

impl<T: ControllerData> Clone for BobaController<T> {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid,
            data: self.data.clone(),
        }
    }
}

impl<T: ControllerData> BobaController<T> {
    pub fn build(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn data(&self) -> &RefCell<T> {
        self.data.as_ref()
    }
}

pub trait ControllerData {}

pub trait ControllerStage<Stage: 'static + BobaStage>: ControllerData {
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}
