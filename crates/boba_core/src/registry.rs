use std::{
    any::{Any, TypeId},
    cell::BorrowMutError,
    hash::Hash,
};

use hashbrown::HashMap;
use indexmap::IndexSet;
use log::{error, info};

use crate::{BobaResources, BobaStage, Pearl, PearlId, PearlMutError, PearlStage, RegisterStages};

/// A collection of pearls, all registered to their respective stages.
///
/// The registry may be told to `run_stage`, and all pearls associated with that stage will be updated.
#[derive(Default)]
pub struct PearlRegistry {
    pearls: HashMap<TypeId, Box<dyn Any>>,
}

impl PearlRegistry {
    pub fn add<T>(&mut self, pearl: &Pearl<T>)
    where
        T: RegisterStages,
    {
        T::register(&pearl, self);
    }

    /// Updates all pearls associated with a specific stage
    pub fn run_stage<Stage>(&mut self, data: &Stage::Data, resources: &mut BobaResources)
    where
        Stage: BobaStage,
    {
        let stageid = TypeId::of::<Stage>();
        let Some(any_collection) = self.pearls.get_mut(&stageid) else {
            info!("PearlRegistry ran stage {}, but there were no associated pearls.", std::any::type_name::<Stage>());
            return;
        };

        any_collection
            .downcast_mut::<PearlCollection<Stage>>()
            .unwrap()
            .update(data, resources);
    }
}

pub trait StageRegistrar {
    fn add<Update, Stage>(&mut self, pearl: Pearl<Update>)
    where
        Stage: BobaStage,
        Update: PearlStage<Stage> + RegisterStages;
}

impl StageRegistrar for PearlRegistry {
    /// Adds a pearl to be tracked by a specific stage in the registry
    fn add<Update, Stage>(&mut self, pearl: Pearl<Update>)
    where
        Stage: BobaStage,
        Update: PearlStage<Stage> + RegisterStages,
    {
        let stageid = TypeId::of::<Stage>();
        match self.pearls.get_mut(&stageid) {
            Some(any_collection) => {
                any_collection
                    .downcast_mut::<PearlCollection<Stage>>()
                    .unwrap()
                    .add(pearl);
            }
            None => {
                let mut collection = PearlCollection::<Stage>::new();
                collection.add(pearl);
                self.pearls.insert(stageid, Box::new(collection));
            }
        }
    }
}

struct PearlCollection<Stage>
where
    Stage: BobaStage,
{
    pearls: IndexSet<Box<dyn PearlRunner<Stage>>>,
}

impl<Stage> PearlCollection<Stage>
where
    Stage: BobaStage,
{
    pub fn new() -> Self {
        Self {
            pearls: Default::default(),
        }
    }

    pub fn add<Update>(&mut self, pearl: Pearl<Update>)
    where
        Update: PearlStage<Stage>,
    {
        self.pearls.insert(Box::new(pearl));
    }

    pub fn update(&mut self, data: &Stage::Data, resources: &mut BobaResources) {
        self.pearls
            .retain(|runner| match runner.dynamic_update(data, resources) {
                PearlStatus::Dead => false,
                _ => true,
            });
    }
}

enum PearlStatus {
    Dead,
    Alive,
    BorrowError(BorrowMutError),
}

trait PearlRunner<Stage>
where
    Stage: BobaStage,
{
    fn id(&self) -> &PearlId;
    fn dynamic_update(&self, data: &Stage::Data, resources: &mut BobaResources) -> PearlStatus;
}

impl<Stage, Update> PearlRunner<Stage> for Pearl<Update>
where
    Stage: BobaStage,
    Update: PearlStage<Stage>,
{
    fn id(&self) -> &PearlId {
        self.id()
    }

    fn dynamic_update(
        &self,
        data: &<Stage as BobaStage>::Data,
        resources: &mut BobaResources,
    ) -> PearlStatus {
        let mut borrow = match self.borrow_mut() {
            Ok(borrow) => borrow,
            Err(PearlMutError::Destroyed) => return PearlStatus::Dead,
            Err(PearlMutError::Borrowed(e)) => {
                error!(
                    "Cannot update Pearl<{}>. Error: {e}",
                    std::any::type_name::<Update>()
                );
                return PearlStatus::BorrowError(e);
            }
        };

        if let Err(e) = borrow.update(data, resources) {
            error!(
                "There was an error while updating Pearl<{}>. Error: {e}",
                std::any::type_name::<Update>()
            );
        };

        return PearlStatus::Alive;
    }
}

impl<Stage> Eq for Box<dyn PearlRunner<Stage>> where Stage: BobaStage {}

impl<Stage> PartialEq for Box<dyn PearlRunner<Stage>>
where
    Stage: BobaStage,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<Stage> Hash for Box<dyn PearlRunner<Stage>>
where
    Stage: BobaStage,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
