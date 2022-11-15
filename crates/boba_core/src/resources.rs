use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    time::Instant,
};

use hashbrown::HashMap;

#[derive(Debug)]
pub enum ResourceError {
    NotFound,
    BorrowedAsMut,
}

#[derive(Default)]
pub struct BobaResources {
    pub(crate) time: BobaTime,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl BobaResources {
    pub fn time(&self) -> &BobaTime {
        &self.time
    }

    pub fn add<T: 'static>(&mut self, item: T) {
        let typeid = TypeId::of::<T>();
        self.resources.insert(typeid, Box::new(RefCell::new(item)));
    }

    pub fn borrow<T: 'static>(&self) -> Result<Ref<T>, ResourceError> {
        let Ok(item) = self.get_ref_cell::<T>()?.try_borrow() else {
            return Err(ResourceError::BorrowedAsMut);
        };
        Ok(item)
    }

    pub fn borrow_mut<T: 'static>(&self) -> Result<RefMut<T>, ResourceError> {
        let Ok(item) = self.get_ref_cell::<T>()?.try_borrow_mut() else {
            return Err(ResourceError::BorrowedAsMut);
        };
        Ok(item)
    }

    fn get_ref_cell<T: 'static>(&self) -> Result<&RefCell<T>, ResourceError> {
        let typeid = TypeId::of::<T>();
        let Some(any) = self.resources.get(&typeid) else {
            return Err(ResourceError::NotFound);
        };
        Ok(any
            .downcast_ref::<RefCell<T>>()
            .expect("Downcast should always succeed if item exists"))
    }
}

pub struct BobaTime {
    delta: f32,
    instant: Instant,
}

impl Default for BobaTime {
    fn default() -> Self {
        Self {
            delta: 0.,
            instant: Instant::now(),
        }
    }
}

impl BobaTime {
    pub(crate) fn reset(&mut self) {
        self.delta = self.instant.elapsed().as_secs_f32();
        self.instant = Instant::now();
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }
}
