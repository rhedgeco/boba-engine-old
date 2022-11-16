use anyhow::Result;
use log::error;
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
    fn wrap(data: T) -> Self {
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

pub trait PearlRunner<Stage>
where
    Stage: 'static + BobaStage,
{
    fn run(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}

impl<Stage, Update> PearlRunner<Stage> for Pearl<Update>
where
    Stage: 'static + BobaStage,
    Update: 'static + BobaUpdate<Stage>,
{
    fn run(&mut self, data: &<Stage as BobaStage>::StageData, resources: &mut BobaResources) {
        match Update::update(data, self, resources) {
            Ok(_) => {}
            Err(error) => error!(
                "There was a(n) {:?} when updating pearl: {:?}",
                error,
                self.uuid()
            ),
        }
    }
}

pub type BobaResult = Result<()>;

pub trait BobaUpdate<Stage>: StageRegister
where
    Stage: 'static + BobaStage,
{
    fn update(
        data: &Stage::StageData,
        pearl: &mut Pearl<Self>,
        resources: &mut BobaResources,
    ) -> BobaResult;
}

pub trait StageRegister
where
    Self: Sized,
{
    fn register(pearl: Pearl<Self>, storage: &mut StageRunners);
}

pub trait PearlWrapper<T> {
    fn pearl(self) -> Pearl<T>;
}

impl<T> PearlWrapper<T> for T {
    fn pearl(self) -> Pearl<T> {
        Pearl::wrap(self)
    }
}
