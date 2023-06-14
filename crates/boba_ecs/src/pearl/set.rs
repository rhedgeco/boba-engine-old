use std::slice::{Iter, IterMut};

use imposters::{collections::vec::ImposterVec, Imposter};

use crate::Pearl;

use super::{
    id::{PearlIdMap, PearlIdSet},
    PearlId,
};

#[derive(Debug, Default)]
pub struct PearlSet {
    map: PearlIdMap<Imposter>,
}

impl PearlSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn id_set(&self) -> &PearlIdSet {
        self.map.id_set()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Option<P> {
        self.map.insert(P::id(), Imposter::new(pearl))?.downcast()
    }

    pub fn remove<P: Pearl>(&mut self) -> Option<P> {
        self.map.remove(&P::id())?.downcast()
    }
}

#[derive(Debug)]
pub struct PearlMatrix {
    len: usize,
    map: PearlIdMap<ImposterVec>,
}

impl PearlMatrix {
    pub fn from_set(set: PearlSet) -> Self {
        let mut map = PearlIdMap::new();
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

    pub fn id_set(&self) -> &PearlIdSet {
        self.map.id_set()
    }

    pub fn push(&mut self, set: PearlSet) {
        if self.id_set() != set.id_set() {
            panic!("Tried to push PearlSet into mismatched PearlMatrix.");
        }

        for (vec, imposter) in self.map.values_mut().zip(set.map.into_values()) {
            vec.push_imposter(imposter).ok().unwrap();
        }

        self.len += 1;
    }

    pub fn swap_remove(&mut self, index: usize) -> PearlSet {
        let mut map = PearlIdMap::new();
        for vec in self.map.values_mut() {
            let imposter = vec.swap_remove(index).expect("Index out of bounds.");
            let type_id = imposter.type_id();
            let pearl_id = PearlId { type_id };
            map.insert(pearl_id, imposter);
        }

        PearlSet { map }
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

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        Some(self.map.get(&P::id())?.as_slice::<P>()?.iter())
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
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
    pub fn new(matrix: &'a mut PearlMatrix) -> Self {
        Self {
            inner: matrix.map.fetch(),
        }
    }

    pub fn get<P: Pearl>(&mut self) -> Option<IterMut<'a, P>> {
        let vec = self.inner.get(&P::id())?;
        Some(vec.as_slice_mut::<P>()?.iter_mut())
    }

    pub unsafe fn get_unmasked<P: Pearl>(&mut self) -> Option<IterMut<'a, P>> {
        let vec = self.inner.get_unmasked(&P::id())?;
        Some(vec.as_slice_mut::<P>()?.iter_mut())
    }
}
