use std::cell::Ref;

use boba_core::{BobaContainer, BobaController};
use indexmap::IndexMap;
use log::error;
use uuid::Uuid;

pub struct TaroStorage<T>
where
    T: BobaController,
{
    controllers: IndexMap<Uuid, BobaContainer<T>>,
}

impl<T> Default for TaroStorage<T>
where
    T: BobaController,
{
    fn default() -> Self {
        Self {
            controllers: Default::default(),
        }
    }
}

impl<T> TaroStorage<T>
where
    T: BobaController,
{
    pub fn add(&mut self, controller: BobaContainer<T>) {
        self.controllers.insert(*controller.uuid(), controller);
    }

    pub fn remove(&mut self, uuid: &Uuid) {
        self.controllers.remove(uuid);
    }

    pub fn collect(&self) -> Vec<Ref<T>> {
        let length = self.controllers.len();
        if length == 0 {
            return Vec::new();
        }

        self.controllers
            .values()
            .filter_map(|f| match f.data().try_borrow() {
                Ok(borrow) => Some(borrow),
                Err(e) => {
                    error!(
                        "Could not collect item BobaController<{:?}> because it is currently mutable borrowed. BorrowError: {:?}",
                        std::any::type_name::<T>(),
                        e,
                    );

                    None
                }
            })
            .collect()
    }
}
