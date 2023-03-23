use imposters::collections::vec::ImposterVec;

use crate::{
    pearls::{PearlIdSet, PearlSet},
    Entity,
};

/// Holds all the data relevant to an entity being removed from an archetype
pub struct ArchRemoved {
    pub set: PearlSet,
    pub swapped: Option<Entity>,
}

/// A collection of entities that share a common pearl set
pub struct Archetype {
    ids: PearlIdSet,
    entities: Vec<Entity>,
    pearls: Vec<ImposterVec>,
}

impl Archetype {
    /// Constructs a new archetype out of and [`Entity`] and a [`PearlSet`]
    pub fn new(entity: Entity, set: PearlSet) -> Self {
        let ids = set.id_set().clone();
        let entities = vec![entity];
        let mut pearls = Vec::new();
        for imposter in set.into_vec().into_iter() {
            pearls.push(ImposterVec::from_imposter(imposter));
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
        for (i, imposter) in set.into_vec().into_iter().enumerate() {
            let push_result = self.pearls[i].push_imposter(imposter);
            push_result.ok().expect("Invalid imposter insert");
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
    /// The pearls and the swapped entity will be returned as `ArchRemoved { set, swapped }`
    pub fn swap_remove(&mut self, index: usize) -> ArchRemoved {
        assert!(index < self.len());

        // remove the entity
        self.entities.swap_remove(index);

        // create a new pearl set to store removed pearls
        let mut set = PearlSet::new();

        // iterate over each collection and remove the pearl at index
        // then insert that pearl into the pearl set
        for vec in self.pearls.iter_mut() {
            let imposter = vec.swap_remove(index).unwrap();
            set.insert_or_replace_imposter(imposter);
        }

        // get the swapped entity id if there is one
        let swapped = self.entities.get(index).copied();

        ArchRemoved { set, swapped }
    }
}
