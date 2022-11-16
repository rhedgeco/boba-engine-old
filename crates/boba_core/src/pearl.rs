use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    rc::Rc,
};

use uuid::Uuid;

use crate::{storage::StageRunners, BobaResources, BobaStage};

pub struct Pearl<T> {
    uuid: Uuid,
    data: Rc<RefCell<T>>,
}

impl<T> Clone for Pearl<T> {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid,
            data: self.data.clone(),
        }
    }
}

impl<T> Pearl<T> {
    pub fn wrap(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn data(&self) -> Result<Ref<T>, BorrowError> {
        self.data.as_ref().try_borrow()
    }

    pub fn data_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.data.as_ref().try_borrow_mut()
    }
}

pub trait BobaUpdate<Stage>: StageRegister
where
    Stage: 'static + BobaStage,
{
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}

pub trait StageRegister
where
    Self: Sized,
{
    fn register(pearl: Pearl<Self>, storage: &mut StageRunners);
}
