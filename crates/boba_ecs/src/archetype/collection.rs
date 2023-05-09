use indexmap::{indexmap, IndexMap};

use crate::{
    entity::EntityCollection,
    pearl::{PearlIdSet, PearlSet},
    Archetype, Entity, Pearl,
};

struct ArchLink {
    arch_index: usize,
    pearl_index: usize,
}

pub struct ArchetypeCollection {
    entities: EntityCollection<ArchLink>,
    archetypes: IndexMap<PearlIdSet, Archetype>,
}

impl Default for ArchetypeCollection {
    fn default() -> Self {
        let empty_ids = PearlIdSet::new();
        let empty_arch = Archetype::new(empty_ids.clone());

        Self {
            entities: Default::default(),
            archetypes: indexmap! {empty_ids => empty_arch},
        }
    }
}

impl ArchetypeCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entity_len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    pub fn entity(&mut self) -> Entity {
        let pearl_index = self.archetypes[0].len();
        self.entities.spawn(ArchLink {
            arch_index: 0,
            pearl_index,
        })
    }

    pub fn insert_pearl<P: Pearl>(&mut self, entity: Entity, pearl: P) {
        self.modify(entity, |mut set| {
            set.insert(pearl);
            set
        });
    }

    pub fn remove_pearl<P: Pearl>(&mut self, entity: Entity) -> Option<P> {
        let mut removed = None;
        self.modify(entity, |mut set| {
            removed = set.remove::<P>();
            set
        });

        removed
    }

    pub fn iter(&self) -> impl Iterator<Item = &Archetype> {
        self.archetypes.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Archetype> {
        self.archetypes.values_mut()
    }

    pub fn modify(&mut self, entity: Entity, modify: impl FnOnce(PearlSet) -> PearlSet) {
        // get the entity archetype and remove its pearl set
        let Some(link) = self.entities.get_data(entity) else { return };
        let archetype = &mut self.archetypes[link.arch_index];
        let swap = archetype.swap_remove(link.pearl_index);

        // fix swapped entity data by setting pearl index to removed index
        if let Some(swap_entity) = swap.swapped_entity {
            self.entities.get_data_mut(swap_entity).unwrap().pearl_index = link.pearl_index;
        }

        // modify the pearl set using the provided function
        let new_set = modify(swap.removed_set);

        // insert the new pearl set back into the world
        let link = match self.archetypes.get_index_of(new_set.ids()) {
            Some(arch_index) => {
                let archetype = &mut self.archetypes[arch_index];
                let pearl_index = archetype.insert(entity, new_set);
                ArchLink {
                    arch_index,
                    pearl_index,
                }
            }
            None => {
                let arch_index = self.archetypes.len();
                let archetype = Archetype::new_with(entity, new_set);
                self.archetypes.insert(archetype.ids().clone(), archetype);
                ArchLink {
                    arch_index,
                    pearl_index: 0,
                }
            }
        };

        // provide the new link to the swapped entity
        *self.entities.get_data_mut(entity).unwrap() = link;
    }
}
