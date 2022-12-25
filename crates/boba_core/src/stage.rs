use std::any::{Any, TypeId};

use indexmap::IndexMap;

use crate::{BobaResources, PearlRegistry};

/// Used for ordered execution of logic and pearl updates
pub trait BobaStage: 'static {
    type Data;
    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources);
}

/// An ordered collection of BobaStages
#[derive(Debug, Default)]
pub struct StageCollection {
    stages: IndexMap<TypeId, Box<dyn Any>>,
}

impl StageCollection {
    /// Adds or replaces a stage in the collection.
    ///
    /// If the stage exists, it will be replaced. If it does not it will be appended.
    pub fn insert<Stage>(&mut self, stage: Stage)
    where
        Stage: BobaStage,
    {
        let stageid = TypeId::of::<Stage>();
        self.stages.insert(stageid, Box::new(stage));
    }

    /// Appends a stage to the collection
    ///
    /// If an instance of this stage already exists in this collection, it will be removed first.
    pub fn append<Stage>(&mut self, stage: Stage)
    where
        Stage: BobaStage,
    {
        let stageid = TypeId::of::<Stage>();
        self.stages.shift_remove(&stageid);
        self.stages.insert(stageid, Box::new(stage));
    }

    /// Prepends a stage to the collection
    ///
    /// If an instance of this stage already exists in this collection, it will be removed first.
    pub fn prepend<Stage>(&mut self, stage: Stage)
    where
        Stage: BobaStage,
    {
        let stageid = TypeId::of::<Stage>();
        self.stages.shift_remove(&stageid);

        let (index, _) = self.stages.insert_full(stageid, Box::new(stage));
        if index > 0 {
            self.stages.move_index(index, 0);
        }
    }

    /// Removes a stage from the collection
    pub fn remove<Stage>(&mut self)
    where
        Stage: BobaStage,
    {
        let stageid = TypeId::of::<Stage>();
        self.stages.shift_remove(&stageid);
    }
}
