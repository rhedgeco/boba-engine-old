use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use uuid::Uuid;

use crate::{BobaResources, BobaStage};

#[derive(Debug, Clone)]
pub struct BobaController<T: ControllerData> {
    uuid: Uuid,
    data: Rc<RefCell<T>>,
}

impl<T: ControllerData> BobaController<T> {
    pub fn build(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn data_mut(&mut self) -> RefMut<T> {
        self.data.borrow_mut()
    }

    pub(crate) unsafe fn direct_mut(&mut self) -> &mut T {
        self.data.as_ptr().as_mut().unwrap()
    }
}

pub trait ControllerData {}

pub trait ControllerStage<Stage: 'static + BobaStage>: ControllerData {
    fn update<'a>(&'a mut self, data: &mut Stage::StageData<'a>, resources: &mut BobaResources);
}
