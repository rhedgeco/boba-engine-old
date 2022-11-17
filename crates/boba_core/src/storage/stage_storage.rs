use indexmap::IndexMap;
use log::warn;
use std::any::TypeId;

use crate::{
    BobaResources, BobaStage, MainBobaUpdate, Pearl, PearlRegister, PearlStage, StageRunner,
};

pub struct StageStorage {
    runners: StageRunners,
}

impl Default for StageStorage {
    fn default() -> Self {
        let mut storage = Self {
            runners: Default::default(),
        };

        storage.insert(MainBobaUpdate);

        storage
    }
}

#[derive(Default)]
pub struct StageRunners {
    stages: IndexMap<TypeId, Box<dyn AnyStageRunner>>,
}

impl StageRunners {
    pub fn add<Stage, Update>(&mut self, pearl: Pearl<Update>)
    where
        Stage: 'static + BobaStage,
        Update: 'static + PearlStage<Stage>,
    {
        let Some(stage_box) = self.stages.get_mut(&TypeId::of::<Stage>()) else {
            warn!("Pearl not added. Stage {:?} was not found", std::any::type_name::<Stage>());
            return;
        };

        stage_box
            .downcast_mut::<StageRunner<Stage>>()
            .expect("Stage runner should be valid at this point")
            .pearls
            .add(pearl);
    }
}

#[derive(Debug)]
pub enum InsertAfterError {
    IdenticalTypes,
    StageNotFound,
}

/// Storage solution for ordered stages
impl StageStorage {
    /// Inserts a `BobaStage` into the storage system
    ///
    /// If the stage already exists in the system, it will be replaced with this new one.
    /// If the stage does not already exist, it is simply appended to the end.
    /// This will take **O(1)** time.
    pub fn insert<Stage>(&mut self, stage: Stage)
    where
        Stage: 'static + BobaStage,
    {
        self.runners
            .stages
            .insert(TypeId::of::<Stage>(), Box::new(StageRunner::build(stage)));
    }

    /// Prepends a stage to the beginning of the list
    ///
    /// This shifts all stages in the list down, and will take **O(n)** time.
    pub fn prepend<Stage>(&mut self, stage: Stage)
    where
        Stage: 'static + BobaStage,
    {
        let (stage_index, _) = self
            .runners
            .stages
            .insert_full(TypeId::of::<Stage>(), Box::new(StageRunner::build(stage)));

        if stage_index > 0 {
            self.runners.stages.move_index(stage_index, 0);
        }
    }

    /// Inserts a stage after the target stage
    ///
    /// This shifts all stages after the insert point over one index.
    /// This will take **O(n)** time.
    ///
    /// ### Error Cases
    /// - **IdenticalTypes**: Cannot insert a stage after itself
    /// - **StageNotFound**: Cannot insert after a stage that doesnt exist
    pub fn insert_after<BeforeStage, AfterStage>(
        &mut self,
        stage: AfterStage,
    ) -> Result<(), InsertAfterError>
    where
        AfterStage: 'static + BobaStage,
        BeforeStage: 'static + BobaStage,
    {
        let before_id = TypeId::of::<BeforeStage>();
        let after_id = TypeId::of::<AfterStage>();

        if before_id == after_id {
            return Err(InsertAfterError::IdenticalTypes);
        }

        let Some(before_index) = self
            .runners
            .stages
            .get_index_of(&before_id) else {
                return Err(InsertAfterError::StageNotFound);
            };

        let (after_index, _) = self
            .runners
            .stages
            .insert_full(after_id, Box::new(StageRunner::build(stage)));

        let target_index = before_index + 1;
        if after_index != target_index {
            self.runners.stages.move_index(after_index, target_index);
        }

        Ok(())
    }

    /// Runs the pearl's `PearlRegister` function to add references to appropriate stages
    pub fn add_pearl<Update>(&mut self, pearl: Pearl<Update>)
    where
        Update: PearlRegister,
    {
        Update::register(pearl, &mut self.runners);
    }

    /// Deletes a stage from storage
    ///
    /// This will shift all later stages back by one index.
    /// This will take **O(n)** time.
    pub fn delete<Stage>(&mut self)
    where
        Stage: 'static + BobaStage,
    {
        self.runners.stages.shift_remove(&TypeId::of::<Stage>());
    }

    pub(crate) fn run(&mut self, resources: &mut BobaResources) {
        for stage in self.runners.stages.values_mut() {
            stage.run(resources);
        }
    }
}

trait AnyStageRunner {
    fn type_id(&self) -> TypeId;
    fn run(&mut self, resources: &mut BobaResources);
}

impl dyn AnyStageRunner {
    #[inline]
    pub fn is<T: 'static + AnyStageRunner>(&self) -> bool {
        let t = TypeId::of::<T>();
        let concrete = self.type_id();
        t == concrete
    }

    #[inline]
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static + AnyStageRunner,
    {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_mut_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn downcast_mut_unchecked<T>(&mut self) -> &mut T
    where
        T: 'static + AnyStageRunner,
    {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &mut *(self as *mut dyn AnyStageRunner as *mut T) }
    }
}

impl<Stage> AnyStageRunner for StageRunner<Stage>
where
    Stage: 'static + BobaStage,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<StageRunner<Stage>>()
    }

    fn run<'a>(&mut self, resources: &mut BobaResources) {
        self.run(resources);
    }
}
