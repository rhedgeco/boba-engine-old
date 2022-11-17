use boba_core::{Pearl, PearlId};
use indexmap::IndexMap;
use log::error;
use std::cell::Ref;

pub struct TaroStorage<T> {
    pearls: IndexMap<PearlId, Pearl<T>>,
}

impl<T> Default for TaroStorage<T> {
    fn default() -> Self {
        Self {
            pearls: Default::default(),
        }
    }
}

impl<T> TaroStorage<T> {
    pub fn add(&mut self, pearl: Pearl<T>) {
        self.pearls.insert(*pearl.id(), pearl);
    }

    pub fn remove(&mut self, id: &PearlId) {
        self.pearls.remove(id);
    }

    pub fn collect(&self) -> Vec<Ref<T>> {
        let length = self.pearls.len();
        if length == 0 {
            return Vec::new();
        }

        self.pearls
            .values()
            .filter_map(|f| match f.data() {
                Ok(borrow) => Some(borrow),
                Err(e) => {
                    error!(
                        "Could not collect item Pearl<{:?}> because it is currently mutable borrowed. BorrowError: {:?}",
                        std::any::type_name::<T>(),
                        e,
                    );

                    None
                }
            })
            .collect()
    }
}
