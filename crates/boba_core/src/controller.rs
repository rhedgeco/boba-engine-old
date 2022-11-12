use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{BobaResources, BobaStage};

pub struct BobaContainer<T: BobaController> {
    uuid: Uuid,
    data: Rc<RefCell<T>>,
}

impl<T: BobaController> Clone for BobaContainer<T> {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid,
            data: self.data.clone(),
        }
    }
}

impl<T: BobaController> BobaContainer<T> {
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

pub trait BobaController {}

pub trait BobaUpdate<Stage: 'static + BobaStage>: BobaController {
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}
