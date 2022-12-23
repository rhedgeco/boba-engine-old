use crate::{storage::StageRunners, BobaResources, BobaStage};
use anyhow::Result;
use log::error;
use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    rc::Rc,
    sync::atomic::AtomicU64,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PearlId {
    _id: u64,
}

impl PearlId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            _id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

pub struct Pearl<T> {
    id: PearlId,
    data: Rc<RefCell<T>>,
}

impl<T> Clone for Pearl<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            data: self.data.clone(),
        }
    }
}

impl<T> Pearl<T> {
    pub fn id(&self) -> &PearlId {
        &self.id
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
    Stage: BobaStage,
{
    fn run(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}

impl<Stage, Update> PearlRunner<Stage> for Pearl<Update>
where
    Stage: BobaStage,
    Update: PearlStage<Stage>,
{
    fn run(&mut self, data: &<Stage as BobaStage>::StageData, resources: &mut BobaResources) {
        let mut pearl = match self.data_mut() {
            Ok(p) => p,
            Err(error) => {
                error!("Could not update pearl due to error: {error}");
                return;
            }
        };

        match pearl.update(data, resources) {
            Ok(_) => {}
            Err(error) => {
                error!(
                    "There wan an error while running pearl {:?}: {error}",
                    std::any::type_name::<Update>()
                );
            }
        }
    }
}

pub type PearlResult = Result<()>;

pub trait PearlStage<Stage>: PearlRegister + 'static
where
    Stage: BobaStage,
{
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources) -> PearlResult;
}

pub trait PearlRegister
where
    Self: Sized,
{
    fn register(pearl: Pearl<Self>, storage: &mut StageRunners);
}

pub trait AsPearl<T> {
    fn into_pearl(self) -> Pearl<T>;
}

impl<T> AsPearl<T> for T {
    fn into_pearl(self) -> Pearl<T> {
        Pearl::<T> {
            id: PearlId::new(),
            data: Rc::new(RefCell::new(self)),
        }
    }
}
