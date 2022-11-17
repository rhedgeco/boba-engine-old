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

        storage.add(MainBobaUpdate);

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

impl StageStorage {
    pub fn get<Stage>(&mut self) -> Option<&mut StageRunner<Stage>>
    where
        Stage: 'static + BobaStage,
    {
        let stage_box = self.runners.stages.get_mut(&TypeId::of::<Stage>())?;

        stage_box.downcast_mut::<StageRunner<Stage>>()
    }

    pub fn add<Stage>(&mut self, stage: Stage)
    where
        Stage: 'static + BobaStage,
    {
        self.runners
            .stages
            .insert(TypeId::of::<Stage>(), Box::new(StageRunner::build(stage)));
    }

    pub fn add_pearl<Update>(&mut self, pearl: Pearl<Update>)
    where
        Update: PearlRegister,
    {
        Update::register(pearl, &mut self.runners);
    }

    pub fn delete<Stage>(&mut self)
    where
        Stage: 'static + BobaStage,
    {
        self.runners.stages.remove(&TypeId::of::<Stage>());
    }

    pub fn run(&mut self, resources: &mut BobaResources) {
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
