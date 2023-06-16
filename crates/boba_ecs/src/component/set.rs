use std::slice::{Iter, IterMut};

use imposters::{collections::vec::ImposterVec, Imposter};

use crate::Component;

use super::{
    id::{ComponentIdMap, ComponentIdSet},
    ComponentId,
};

#[derive(Debug, Default)]
pub struct ComponentSet {
    map: ComponentIdMap<Imposter>,
}

impl ComponentSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn id_set(&self) -> &ComponentIdSet {
        self.map.id_set()
    }

    pub fn insert<T: Component>(&mut self, component: T) -> Option<T> {
        self.map
            .insert(T::id(), Imposter::new(component))?
            .downcast()
    }

    pub fn remove<T: Component>(&mut self) -> Option<T> {
        self.map.remove(&T::id())?.downcast()
    }
}

#[derive(Debug)]
pub struct ComponentMatrix {
    len: usize,
    map: ComponentIdMap<ImposterVec>,
}

impl ComponentMatrix {
    pub fn from_set(set: ComponentSet) -> Self {
        let mut map = ComponentIdMap::new();
        for (id, imposter) in set.map.into_iter() {
            map.insert(id, ImposterVec::from_imposter(imposter));
        }

        Self { len: 1, map }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn id_set(&self) -> &ComponentIdSet {
        self.map.id_set()
    }

    pub fn push(&mut self, set: ComponentSet) {
        if self.id_set() != set.id_set() {
            panic!("Tried to push ComponentSet into mismatched ComponentMatrix.");
        }

        for (vec, imposter) in self.map.values_mut().zip(set.map.into_values()) {
            vec.push_imposter(imposter).ok().unwrap();
        }

        self.len += 1;
    }

    pub fn swap_remove(&mut self, index: usize) -> ComponentSet {
        let mut map = ComponentIdMap::new();
        for vec in self.map.values_mut() {
            let imposter = vec.swap_remove(index).expect("Index out of bounds.");
            let type_id = imposter.type_id();
            let component_id = ComponentId { type_id };
            map.insert(component_id, imposter);
        }

        ComponentSet { map }
    }

    pub fn swap_drop(&mut self, index: usize) {
        if index >= self.len {
            panic!("Index out of bounds.");
        }

        for vec in self.map.values_mut() {
            vec.swap_drop(index);
        }

        self.len -= 1;
    }

    pub fn iter<P: Component>(&self) -> Option<Iter<P>> {
        Some(self.map.get(&P::id())?.as_slice::<P>()?.iter())
    }

    pub fn iter_mut<P: Component>(&mut self) -> Option<IterMut<P>> {
        Some(self.map.get_mut(&P::id())?.as_slice_mut::<P>()?.iter_mut())
    }

    pub fn fetch_iter(&mut self) -> IterFetcher {
        IterFetcher::new(self)
    }
}

pub struct IterFetcher<'a> {
    inner: super::id::map::Fetcher<'a, ImposterVec>,
}

impl<'a> IterFetcher<'a> {
    pub fn new(matrix: &'a mut ComponentMatrix) -> Self {
        Self {
            inner: matrix.map.fetch(),
        }
    }

    pub fn get<T: Component>(&mut self) -> Option<IterMut<'a, T>> {
        let vec = self.inner.get(&T::id())?;
        Some(vec.as_slice_mut::<T>()?.iter_mut())
    }

    pub unsafe fn get_unmasked<T: Component>(&mut self) -> Option<IterMut<'a, T>> {
        let vec = self.inner.get_unmasked(&T::id())?;
        Some(vec.as_slice_mut::<T>()?.iter_mut())
    }
}
