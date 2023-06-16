use super::ComponentId;

pub type Iter<'a> = std::slice::Iter<'a, ComponentId>;
pub type IntoIter = std::vec::IntoIter<ComponentId>;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum FindOrInsert {
    Found(usize),
    Inserted(usize),
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentIdSet {
    ids: Vec<ComponentId>,
}

impl IntoIterator for ComponentIdSet {
    type Item = ComponentId;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.ids.into_iter()
    }
}

impl ComponentIdSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn iter(&self) -> Iter {
        self.ids.iter()
    }

    pub fn find(&self, id: &ComponentId) -> Option<usize> {
        self.ids.binary_search(id).ok()
    }

    pub fn insert(&mut self, id: ComponentId) {
        if let Err(index) = self.ids.binary_search(&id) {
            self.ids.insert(index, id);
        }
    }

    pub fn find_or_insert(&mut self, id: &ComponentId) -> FindOrInsert {
        match self.ids.binary_search(id) {
            Ok(index) => FindOrInsert::Found(index),
            Err(index) => {
                self.ids.insert(index, *id);
                FindOrInsert::Inserted(index)
            }
        }
    }

    pub fn drop(&mut self, id: &ComponentId) -> Option<usize> {
        match self.ids.binary_search(id) {
            Ok(index) => {
                self.ids.remove(index);
                Some(index)
            }
            _ => None,
        }
    }

    pub fn remove_index(&mut self, index: usize) -> ComponentId {
        self.ids.remove(index)
    }

    pub fn is_superset(&self, other: &ComponentIdSet) -> bool {
        other.is_subset(self)
    }

    pub fn is_subset(&self, other: &ComponentIdSet) -> bool {
        // early return if other has more items than self.
        // this makes it automatically not a subset.
        if self.len() < other.len() {
            return false;
        }

        // get an iterator over the other ids and obtain the first id in the iterator
        let mut other_iter = other.iter();
        let Some(mut other_id) = other_iter.next() else { return true };

        // loop over every id in self, and match them to the other sets ids
        for self_id in self.iter() {
            // compare self to other id
            match self_id.cmp(other_id) {
                // If less than we have not found the other id yet. Do nothing and continue.
                std::cmp::Ordering::Less => (),

                // If equal to, then we have found the id. Advance the other iterator and continue.
                std::cmp::Ordering::Equal => {
                    // if there are no more item in the other iterator,
                    // then we completed all searches and know that other is a subset. Return true.
                    let Some(next) = other_iter.next() else { return true };
                    other_id = next;
                }

                // If greater than, then the id can never be found in this set. Early return false.
                std::cmp::Ordering::Greater => return false,
            }
        }

        // if we searched all items in self and didnt complete the other iter
        // then other iter has more items to match and it is not a subset. Return false.
        false
    }
}
