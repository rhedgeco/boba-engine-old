use std::ops::{AddAssign, SubAssign};

use fxhash::FxHashSet;
use imposters::collections::vec::ImposterVec;
use indexmap::IndexSet;

use crate::{
    pearl::{PearlId, PearlIdSet, PearlSet},
    Entity, Pearl,
};

pub struct ArchSwap {
    pub removed_set: PearlSet,
    pub swapped_entity: Option<Entity>,
}

pub struct Archetype {
    ids: PearlIdSet,
    entities: IndexSet<Entity>,
    pearls: Vec<ImposterVec>,
    count: usize,
}

impl Archetype {
    pub fn new(ids: PearlIdSet) -> Self {
        let id_len = ids.len();
        Self {
            ids,
            entities: IndexSet::with_capacity(id_len),
            pearls: Vec::with_capacity(id_len),
            count: 0,
        }
    }

    pub fn new_with(entity: Entity, set: PearlSet) -> Self {
        let ids = set.ids().clone();
        let mut entities = IndexSet::new();
        entities.insert(entity);
        let mut pearls = Vec::with_capacity(set.len());
        for imposter in set.pearls {
            pearls.push(ImposterVec::from_imposter(imposter));
        }

        Self {
            ids,
            entities,
            pearls,
            count: 1,
        }
    }

    pub fn ids(&self) -> &PearlIdSet {
        &self.ids
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get<P: Pearl>(&self) -> Option<&[P]> {
        let index = self.ids.contains::<P>()?;
        self.pearls[index].as_slice::<P>()
    }

    pub fn get_mut<P: Pearl>(&mut self) -> Option<&mut [P]> {
        let index = self.ids.contains::<P>()?;
        self.pearls[index].as_slice_mut::<P>()
    }

    pub fn insert(&mut self, entity: Entity, set: PearlSet) -> usize {
        if &self.ids != set.ids() {
            panic!("Tried inserting mismatched PearlSet into Archetype");
        }

        self.entities.insert(entity);
        let insert_index = self.len();
        match self.pearls.len() {
            0 => self.fresh_insert(set),
            _ => self.next_insert(set),
        }

        insert_index
    }

    fn fresh_insert(&mut self, set: PearlSet) {
        self.pearls.clear();
        for imposter in set.pearls {
            let vec = ImposterVec::from_imposter(imposter);
            self.pearls.push(vec);
        }

        self.count = 1;
    }

    fn next_insert(&mut self, set: PearlSet) {
        for (index, imposter) in set.pearls.into_iter().enumerate() {
            self.pearls[index]
                .push_imposter(imposter)
                .ok()
                .expect("Imposter should be valid");
        }

        self.count.add_assign(1);
    }

    pub fn swap_remove(&mut self, index: usize) -> ArchSwap {
        let mut pearls = Vec::new();
        for imposters in self.pearls.iter_mut() {
            let imposter = imposters.swap_remove(index).expect("Index out of bounds");
            pearls.push(imposter);
        }

        self.entities.swap_remove_index(index);
        let swapped = self.entities.get_index(index);
        let set = PearlSet {
            ids: self.ids.clone(),
            pearls,
        };

        self.count.sub_assign(1);
        ArchSwap {
            removed_set: set,
            swapped_entity: swapped.cloned(),
        }
    }

    pub fn swap_drop(&mut self, index: usize) -> Option<Entity> {
        for imposters in self.pearls.iter_mut() {
            imposters.swap_drop(index);
        }

        self.count.sub_assign(1);
        self.entities.swap_remove_index(index);
        self.entities.get_index(index).cloned()
    }

    pub fn start_fetching(&mut self) -> PearlFetcher {
        PearlFetcher {
            fetched: Default::default(),
            archetype: self,
        }
    }
}

pub struct PearlFetcher<'a> {
    fetched: FxHashSet<PearlId>,
    archetype: &'a mut Archetype,
}

impl<'a> PearlFetcher<'a> {
    pub fn fetch<P: Pearl>(&mut self) -> Option<&'a mut [P]> {
        if !self.fetched.insert(P::id()) {
            return None;
        }

        let slice = self.archetype.get_mut::<P>()?;

        // SAFETY: Transmuting here does some messy things to the lifetime.
        // However, since each particular item is exluded from future calls using the fetched set,
        // none of them may be accessed twice which would result in multiple mutable access.
        Some(unsafe { std::mem::transmute(slice) })
    }
}
