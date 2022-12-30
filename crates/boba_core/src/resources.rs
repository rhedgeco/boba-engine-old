use std::{
    any::{Any, TypeId},
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
};

use hashbrown::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError<T> {
    #[error("Resource does not exist")]
    NotFound,
    #[error("Error when borrowing resource: {0}")]
    BorrowError(T),
}

#[derive(Default)]
pub struct BobaResources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl BobaResources {
    pub fn get<T: 'static>(&self) -> Result<Ref<T>, ResourceError<BorrowError>> {
        let Some(any) = self.resources.get(&TypeId::of::<T>()) else {
            return Err(ResourceError::NotFound);
        };

        return match any
            .as_ref()
            .downcast_ref::<RefCell<T>>()
            .unwrap()
            .try_borrow()
        {
            Ok(item) => Ok(item),
            Err(borrow) => Err(ResourceError::BorrowError(borrow)),
        };
    }

    pub fn get_mut<T: 'static>(&self) -> Result<RefMut<T>, ResourceError<BorrowMutError>> {
        let Some(any) = self.resources.get(&TypeId::of::<T>()) else {
            return Err(ResourceError::NotFound);
        };

        return match any
            .as_ref()
            .downcast_ref::<RefCell<T>>()
            .unwrap()
            .try_borrow_mut()
        {
            Ok(item) => Ok(item),
            Err(borrow) => Err(ResourceError::BorrowError(borrow)),
        };
    }

    pub fn add<T>(&mut self, resource: T)
    where
        T: 'static,
    {
        self.resources
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(resource)));
    }

    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: 'static,
    {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        Some(any.downcast::<RefCell<T>>().unwrap().into_inner())
    }
}

#[cfg(test)]
mod tests {
    use crate::BobaResources;

    struct TestStruct1;
    struct TestStruct2;

    #[test]
    fn add() {
        let mut resources = BobaResources::default();
        resources.add(TestStruct1);
        assert!(resources.resources.len() == 1);
    }

    #[test]
    fn borrow() {
        let mut resources = BobaResources::default();
        resources.add(TestStruct1);
        assert!(resources.get::<TestStruct1>().is_ok());
        assert!(resources.get::<TestStruct2>().is_err());
        assert!(resources.get_mut::<TestStruct1>().is_ok());
        assert!(resources.get_mut::<TestStruct2>().is_err());
    }

    #[test]
    fn remove() {
        let mut resources = BobaResources::default();
        resources.add(TestStruct2);
        assert!(resources.remove::<TestStruct1>().is_none());
        assert!(resources.remove::<TestStruct2>().is_some());
        assert!(resources.get::<TestStruct2>().is_err());
    }
}
