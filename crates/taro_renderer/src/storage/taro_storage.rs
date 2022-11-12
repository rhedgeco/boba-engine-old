use std::cell::Ref;

use boba_core::{BobaController, ControllerData};
use indexmap::IndexMap;
use uuid::Uuid;

pub struct TaroStorage<T>
where
    T: ControllerData,
{
    controllers: IndexMap<Uuid, BobaController<T>>,
}

impl<T> Default for TaroStorage<T>
where
    T: ControllerData,
{
    fn default() -> Self {
        Self {
            controllers: Default::default(),
        }
    }
}

impl<T> TaroStorage<T>
where
    T: ControllerData,
{
    pub fn add(&mut self, controller: BobaController<T>) {
        self.controllers.insert(*controller.uuid(), controller);
    }

    pub fn remove(&mut self, uuid: &Uuid) {
        self.controllers.remove(uuid);
    }

    pub fn collect(&mut self) -> Vec<Ref<T>> {
        let length = self.controllers.len();
        if length == 0 {
            return Vec::new();
        }

        self.controllers
            .values_mut()
            .map(|f| f.data().borrow())
            .collect()
    }
}
