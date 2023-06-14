use std::any::{Any, TypeId};

use fxhash::{FxHashMap, FxHashSet};

/// A generic storage solution for holding items in boba engine.
/// It can only hold one of each item, so each item is a kind of singleton.
#[derive(Default)]
pub struct BobaResources {
    resources: FxHashMap<TypeId, Box<dyn Any>>,
}

impl BobaResources {
    /// Returns a new resource map
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces `resource` into this map.
    ///
    /// If a resource of the same type already existed, it is returned as `Some(T)`.
    /// Otherwise `None` is returned.
    #[inline]
    pub fn insert<T: 'static>(&mut self, resource: T) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let old = self.resources.insert(type_id, Box::new(resource))?;
        Some(*old.downcast::<T>().unwrap())
    }

    /// Returns a reference to the resource of type `T` stored in this map.
    ///
    /// Returns `None` if the resource does not exist.
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let any = self.resources.get(&TypeId::of::<T>())?;
        Some(any.downcast_ref::<T>().unwrap())
    }

    /// Gets a reference to `T` if it exists, then runs the function `f`.
    pub fn get_and<T: 'static>(&self, f: impl FnOnce(&T)) {
        let Some(item) = self.get::<T>() else { return };
        f(item);
    }

    /// Returns a mutable reference to the resource of type `T` stored in this map.
    ///
    /// Returns `None` if the resource does not exist.
    #[inline]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let any = self.resources.get_mut(&TypeId::of::<T>())?;
        Some(any.downcast_mut::<T>().unwrap())
    }

    /// Gets a mutable reference to `T` if it exists, then runs the function `f`.
    pub fn get_mut_and<T: 'static>(&mut self, f: impl FnOnce(&mut T)) {
        let Some(item) = self.get_mut::<T>() else { return };
        f(item);
    }

    /// Removes and returns the resource of type `T` stored in this map.
    ///
    /// Returns `None` if the resource does not exist.
    #[inline]
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        Some(*any.downcast::<T>().unwrap())
    }

    pub fn fetch(&mut self) -> ResourceFetcher {
        ResourceFetcher {
            fetched: Default::default(),
            resources: self,
        }
    }
}

pub struct ResourceFetcher<'a> {
    fetched: FxHashSet<TypeId>,
    resources: &'a mut BobaResources,
}

impl<'a> ResourceFetcher<'a> {
    pub fn get<T: 'static>(&mut self) -> Option<&'a mut T> {
        if !self.fetched.insert(TypeId::of::<T>()) {
            return None;
        }

        let resource = self.resources.get_mut::<T>()?;

        // SAFETY: Transmuting here does some messy things to the lifetime.
        // However, since each particular item is exluded from future calls using the fetched set,
        // none of them may be accessed twice which would result in multiple mutable access.
        Some(unsafe { std::mem::transmute(resource) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct1(u64);
    struct TestStruct2(u128);
    struct TestStruct3(u32);

    #[test]
    fn insert_get() {
        let mut res = BobaResources::new();

        assert!(res.insert(TestStruct1(1)).is_none());
        assert!(res.insert(TestStruct2(2)).is_none());
        assert!(res.insert(TestStruct1(3)).is_some());

        assert!(res.get::<TestStruct1>().is_some());
        assert!(res.get::<TestStruct2>().is_some());
        assert!(res.get::<TestStruct3>().is_none());
    }

    #[test]
    fn fetch() {
        let mut res = BobaResources::new();
        res.insert(TestStruct1(5));
        res.insert(TestStruct2(10));

        let mut fetcher = res.fetch();
        // let mut fetcher2 = res.fetch(); // this should fail to compile
        // res.insert(TestStruct3(20)); // this should fail to compile

        let test1 = fetcher.get::<TestStruct1>();
        let test2 = fetcher.get::<TestStruct2>();
        let test3 = fetcher.get::<TestStruct3>();

        assert!(test1.unwrap().0 == 5);
        assert!(test2.unwrap().0 == 10);
        assert!(test3.is_none());
    }
}
