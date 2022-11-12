use std::any::TypeId;

use indexmap::IndexMap;
use log::warn;

use crate::{BobaContainer, BobaResources, BobaStage, BobaUpdate, MainBobaUpdate, StageRunner};

pub struct StageStorage {
    stages: IndexMap<TypeId, Box<dyn AnyStageRunner>>,
}

impl Default for StageStorage {
    fn default() -> Self {
        let mut storage = Self {
            stages: Default::default(),
        };

        storage.add(MainBobaUpdate);

        storage
    }
}

impl StageStorage {
    pub fn get<Stage>(&mut self) -> Option<&mut StageRunner<Stage>>
    where
        Stage: 'static + BobaStage,
    {
        let stage_box = self.stages.get_mut(&TypeId::of::<Stage>())?;

        stage_box.downcast_mut::<StageRunner<Stage>>()
    }

    pub fn add<Stage>(&mut self, stage: Stage)
    where
        Stage: 'static + BobaStage,
    {
        self.stages
            .insert(TypeId::of::<Stage>(), Box::new(StageRunner::build(stage)));
    }

    pub fn add_controller<Stage, Controller>(&mut self, controller: BobaContainer<Controller>)
    where
        Stage: 'static + BobaStage,
        Controller: 'static + BobaUpdate<Stage>,
    {
        let Some(stage_box) = self.stages.get_mut(&TypeId::of::<Stage>()) else {
            warn!("Controller not added. Stage {:?} was not found", std::any::type_name::<Stage>());
            return;
        };

        stage_box
            .downcast_mut::<StageRunner<Stage>>()
            .expect("Stage runner should be valid at this point")
            .controllers()
            .add(controller);
    }

    pub fn delete<Stage>(&mut self)
    where
        Stage: 'static + BobaStage,
    {
        self.stages.remove(&TypeId::of::<Stage>());
    }

    pub fn run(&mut self, resources: &mut BobaResources) {
        for stage in self.stages.values_mut() {
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
    pub fn downcast_mut<T: 'static + AnyStageRunner>(&mut self) -> Option<&mut T> {
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
    pub unsafe fn downcast_mut_unchecked<T: 'static + AnyStageRunner>(&mut self) -> &mut T {
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
