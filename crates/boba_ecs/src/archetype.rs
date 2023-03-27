use indexmap::{map::Entry, IndexMap};

use crate::{
    pearls::{AnyPearlVec, PearlIdSet, PearlSet},
    Entity,
};

/// A collection of entities that share a common pearl set
pub struct Archetype {
    ids: PearlIdSet,
    entities: Vec<Entity>,
    pearls: Vec<AnyPearlVec>,
}

impl Archetype {
    /// Constructs a new archetype out of and [`Entity`] and a [`PearlSet`]
    pub fn new(entity: Entity, set: PearlSet) -> Self {
        let ids = set.id_set().clone();
        let entities = vec![entity];
        let mut pearls = Vec::new();
        for any in set.into_vec().into_iter() {
            pearls.push(AnyPearlVec::from_any(any));
        }

        Self {
            ids,
            entities,
            pearls,
        }
    }

    /// Returns the number of entities in this archetype
    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns true if the archetype is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Inserts an [`Entity`] and [`PearlSet`] into this archetype, returning unique index where they are stored
    ///
    /// This does no validation checking for if the entity already exists in the set,
    /// and will happily store duplicate entities. So the onus is on the developer to handle those situations.
    ///
    /// # Panics
    /// This will panic if given a [`PearlSet`] that does not match the existing layout.
    pub fn insert(&mut self, entity: Entity, set: PearlSet) -> usize {
        assert!(&self.ids == set.id_set());
        let new_index = self.len();
        self.entities.push(entity);
        for (i, any) in set.into_vec().into_iter().enumerate() {
            let push_result = self.pearls[i].push_any(any);
            push_result.ok().expect("Invalid AnyPearl insert");
        }
        new_index
    }

    /// Destroys an entity and its pearls at a given index, swapping them for the last entity in the layout.
    /// The entity that was swapped into the open index will be returned as `Some(Entity)`.
    ///
    /// In the event that there are no more entities left in the archetype,
    /// or the removed entity was the last in the archetype, `None` will be returned.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    pub fn swap_destroy(&mut self, index: usize) -> Option<Entity> {
        assert!(index < self.len());
        self.entities.swap_remove(index);
        for vec in self.pearls.iter_mut() {
            vec.swap_drop(index);
        }

        self.entities.get(index).copied()
    }

    /// Removes an entity and its pearls at a given index, swapping them for the last entity in the layout.
    /// The pearls and the swapped entity will be returned as `( set, swapped )`
    ///
    /// See `swap_destroy` for reference on entity swapping.
    pub fn swap_remove(&mut self, index: usize) -> (PearlSet, Option<Entity>) {
        assert!(index < self.len());

        // remove the entity
        self.entities.swap_remove(index);

        // create a new pearl set to store removed pearls
        let mut set = PearlSet::new();

        // iterate over each collection and remove the pearl at index
        // then insert that pearl into the pearl set
        for vec in self.pearls.iter_mut() {
            let imposter = vec.swap_remove(index).unwrap();
            set.insert_or_replace_any_pearl(imposter);
        }

        // get the swapped entity id if there is one
        let swapped = self.entities.get(index).copied();

        (set, swapped)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ArchetypeIndexer {
    pub archetype: usize,
    pub pearl_offset: usize,
}

#[derive(Default)]
pub struct ArchetypeManager {
    archetypes: IndexMap<PearlIdSet, Archetype>,
}

impl ArchetypeManager {
    /// Returns a new empty archetype manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts `entity` into the archetype that corresponds to `set`
    ///
    /// Returns an [`ArchetypeIndexer`] with the locations of the inserted data
    pub fn insert(&mut self, entity: Entity, set: PearlSet) -> ArchetypeIndexer {
        match self.archetypes.entry(set.id_set().clone()) {
            Entry::Occupied(e) => {
                let archetype_index = e.index();
                let archetype = e.into_mut();
                let pearl_index = archetype.insert(entity, set);
                ArchetypeIndexer {
                    archetype: archetype_index,
                    pearl_offset: pearl_index,
                }
            }
            Entry::Vacant(e) => {
                let archetype = Archetype::new(entity, set);
                let archetype_index = e.index();
                e.insert(archetype);
                ArchetypeIndexer {
                    archetype: archetype_index,
                    pearl_offset: 0,
                }
            }
        }
    }

    #[inline]
    pub fn contains_archetype(&self, ids: &PearlIdSet) -> bool {
        self.archetypes.contains_key(ids)
    }

    #[inline]
    pub fn get(&self, ids: &PearlIdSet) -> Option<&Archetype> {
        self.archetypes.get(ids)
    }

    #[inline]
    pub fn get_mut(&mut self, ids: &PearlIdSet) -> Option<&mut Archetype> {
        self.archetypes.get_mut(ids)
    }

    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&Archetype> {
        Some(self.archetypes.get_index(index)?.1)
    }

    #[inline]
    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut Archetype> {
        Some(self.archetypes.get_index_mut(index)?.1)
    }

    #[inline]
    pub fn swap_remove(&mut self, indexer: &ArchetypeIndexer) -> (PearlSet, Option<Entity>) {
        let archetype = &mut self.archetypes[indexer.archetype];
        archetype.swap_remove(indexer.pearl_offset)
    }

    #[inline]
    pub fn swap_destroy(&mut self, indexer: &ArchetypeIndexer) -> Option<Entity> {
        let archetype = &mut self.archetypes[indexer.archetype];
        archetype.swap_destroy(indexer.pearl_offset)
    }
}
