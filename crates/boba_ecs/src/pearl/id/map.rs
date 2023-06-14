use std::iter::Zip;

use super::{PearlId, PearlIdSet};

pub type Ids<'a> = super::set::Iter<'a>;
pub type IntoIds = super::set::IntoIter;
pub type Values<'a, T> = std::slice::Iter<'a, T>;
pub type ValuesMut<'a, T> = std::slice::IterMut<'a, T>;
pub type IntoValues<T> = std::vec::IntoIter<T>;
pub type Iter<'a, T> = Zip<Ids<'a>, Values<'a, T>>;
pub type IterMut<'a, T> = Zip<Ids<'a>, ValuesMut<'a, T>>;
pub type IntoIter<T> = Zip<IntoIds, IntoValues<T>>;

#[derive(Debug)]
pub struct PearlIdMap<T> {
    set: PearlIdSet,
    values: Vec<T>,
}

impl<T> Default for PearlIdMap<T> {
    fn default() -> Self {
        Self {
            set: Default::default(),
            values: Default::default(),
        }
    }
}

impl<T> IntoIterator for PearlIdMap<T> {
    type Item = (PearlId, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter().zip(self.values.into_iter())
    }
}

impl<T> PearlIdMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn id_set(&self) -> &PearlIdSet {
        &self.set
    }

    pub fn ids(&self) -> Ids {
        self.set.iter()
    }

    pub fn into_ids(self) -> IntoIds {
        self.set.into_iter()
    }

    pub fn values(&self) -> Values<T> {
        self.values.iter()
    }

    pub fn values_mut(&mut self) -> ValuesMut<T> {
        self.values.iter_mut()
    }

    pub fn into_values(self) -> IntoValues<T> {
        self.values.into_iter()
    }

    pub fn iter(&self) -> Iter<T> {
        self.set.iter().zip(self.values.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.set.iter().zip(self.values.iter_mut())
    }

    pub fn get(&self, id: &PearlId) -> Option<&T> {
        let index = self.set.find(id)?;
        Some(&self.values[index])
    }

    pub fn get_mut(&mut self, id: &PearlId) -> Option<&mut T> {
        let index = self.set.find(id)?;
        Some(&mut self.values[index])
    }

    pub fn fetch(&mut self) -> Fetcher<T> {
        Fetcher::new(self)
    }

    pub fn insert(&mut self, id: PearlId, value: T) -> Option<T> {
        use crate::pearl::id::set::FindOrInsert::*;
        match self.set.find_or_insert(&id) {
            Found(index) => Some(std::mem::replace(&mut self.values[index], value)),
            Inserted(index) => {
                self.values.insert(index, value);
                None
            }
        }
    }

    pub fn remove(&mut self, id: &PearlId) -> Option<T> {
        match self.set.drop(id) {
            Some(index) => Some(self.values.remove(index)),
            None => None,
        }
    }
}

pub struct Fetcher<'a, T> {
    mask: PearlIdSet,
    map: &'a mut PearlIdMap<T>,
}

impl<'a, T> Fetcher<'a, T> {
    pub fn new(map: &'a mut PearlIdMap<T>) -> Self {
        Self {
            mask: PearlIdSet::new(),
            map,
        }
    }

    pub fn get(&mut self, id: &PearlId) -> Option<&'a mut T> {
        use crate::pearl::id::set::FindOrInsert::*;
        if let Found(_) = self.mask.find_or_insert(id) {
            return None;
        }

        unsafe { self.get_unmasked(id) }
    }

    pub unsafe fn get_unmasked(&mut self, id: &PearlId) -> Option<&'a mut T> {
        let value = self.map.get_mut(id)?;
        Some(unsafe { std::mem::transmute(value) })
    }
}
