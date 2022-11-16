use hashbrown::HashMap;
use log::error;
use uuid::Uuid;

use crate::{BobaResources, BobaStage, BobaUpdate, Pearl};

/// A storage solution for `Pearl` objects.
///
/// You may be `insert` and `remove` pearls in any order.
/// The storage system has an `update` function that will iterate over
/// every component and call its corresponding `update` function.
/// This struct will typically be used inside a `BobaStage` as the
/// owner of all the pearls to be run for that stage.
pub struct PearlStorage<Stage: 'static + ?Sized + BobaStage> {
    pearls: HashMap<Uuid, Box<dyn GenericBobaStage<Stage>>>,
}

/// The default implementation for `PearlStorage<BobaStage>`
impl<Stage: 'static + BobaStage> Default for PearlStorage<Stage> {
    fn default() -> Self {
        Self {
            pearls: Default::default(),
        }
    }
}

impl<Stage: 'static + BobaStage> PearlStorage<Stage> {
    /// Adds a pearl to the storage system.
    pub fn add<T>(&mut self, pearl: Pearl<T>)
    where
        T: 'static + BobaUpdate<Stage>,
    {
        self.pearls.insert(*pearl.uuid(), Box::new(pearl));
    }

    /// Removed a pearl from the storage system
    pub fn remove(&mut self, uuid: &Uuid) {
        self.pearls.remove(uuid);
    }

    // updates all pearls that are currently in storage
    pub fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources) {
        for pearl in self.pearls.values_mut() {
            pearl.update(data, resources);
        }
    }
}

trait GenericBobaStage<Stage: 'static + BobaStage> {
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}

impl<Stage, Update> GenericBobaStage<Stage> for Pearl<Update>
where
    Stage: 'static + BobaStage,
    Update: BobaUpdate<Stage>,
{
    fn update<'a>(&'a mut self, data: &Stage::StageData, resources: &mut BobaResources) {
        let Ok(mut pearl) = self.data().try_borrow_mut() else {
            error!("Skipping update on Pearl<{:?}>: BorrowMutError. Already Borrowed", std::any::type_name::<Update>());
            return;
        };

        pearl.update(data, resources);
    }
}
