use std::any::{Any, TypeId};

use hashbrown::HashMap;
use log::{error, warn};

use crate::{BobaResources, BobaStage, Pearl, PearlStage, RegisterStages};

#[derive(Debug, Default)]
pub struct PearlRegistry {
    pearls: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl PearlRegistry {
    pub fn add_pearl<T>(&mut self, pearl: &Pearl<T>)
    where
        T: RegisterStages,
    {
        T::register(&pearl, self);
    }

    pub fn run_stage<Stage>(&self, data: &Stage::Data, resources: &mut BobaResources)
    where
        Stage: BobaStage,
    {
        let stageid = TypeId::of::<Stage>();
        let Some(any_runner_vec) = self.pearls.get(&stageid) else {
            warn!("PearlRegistry ran stage {}, but there were no associated pearls.", std::any::type_name::<Stage>());
            return;
        };

        for any_runner in any_runner_vec.iter() {
            let boxed_runner = any_runner
                .downcast_ref::<Box<dyn PearlRunner<Stage>>>()
                .unwrap();
            boxed_runner.dynamic_update(data, resources);
        }
    }
}

pub trait StageRegistrar {
    fn add<Update, Stage>(&mut self, pearl: Pearl<Update>)
    where
        Stage: BobaStage,
        Update: PearlStage<Stage> + RegisterStages;
}

impl StageRegistrar for PearlRegistry {
    fn add<Update, Stage>(&mut self, pearl: Pearl<Update>)
    where
        Stage: BobaStage,
        Update: PearlStage<Stage> + RegisterStages,
    {
        let stageid = TypeId::of::<Stage>();
        let boxed_runner: Box<dyn PearlRunner<Stage>> = Box::new(pearl);
        let any_runner: Box<dyn Any> = Box::new(boxed_runner);
        match self.pearls.get_mut(&stageid) {
            Some(vec) => vec.push(any_runner),
            None => {
                let mut vec = Vec::new();
                vec.push(any_runner);
                self.pearls.insert(stageid, vec);
            }
        }
    }
}

trait PearlRunner<Stage>
where
    Stage: BobaStage,
{
    fn dynamic_update(&self, data: &Stage::Data, resources: &mut BobaResources);
}

impl<Stage, Update> PearlRunner<Stage> for Pearl<Update>
where
    Stage: BobaStage,
    Update: PearlStage<Stage>,
{
    fn dynamic_update(&self, data: &<Stage as BobaStage>::Data, resources: &mut BobaResources) {
        let mut borrow = match self.borrow_mut() {
            Ok(borrow) => borrow,
            Err(e) => {
                error!(
                    "Cannot update Pearl<{}>. Error: {e}",
                    std::any::type_name::<Update>()
                );
                return;
            }
        };

        if let Err(e) = borrow.update(data, resources) {
            error!(
                "There was an error while updating Pearl<{}>. Error: {e}",
                std::any::type_name::<Update>()
            );
            return;
        };
    }
}
