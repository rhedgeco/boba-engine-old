use imposters::Imposter;

use super::{Pearl, PearlId, PearlIdSet};

#[derive(Default)]
pub struct PearlSet {
    pub(crate) ids: PearlIdSet,
    pub(crate) pearls: Vec<Imposter>,
}

impl PearlSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with<P: Pearl>(pearl: P) -> Self {
        Self {
            ids: PearlIdSet::new_with::<P>(),
            pearls: vec![Imposter::new(pearl)],
        }
    }

    pub fn ids(&self) -> &PearlIdSet {
        &self.ids
    }

    pub fn len(&self) -> usize {
        self.pearls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pearls.is_empty()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) {
        let imposter = Imposter::new(pearl);
        match self.ids.insert::<P>() {
            Ok(_) => self.pearls.push(imposter),
            Err(index) => self.pearls[index] = imposter,
        }
    }

    pub fn remove<P: Pearl>(&mut self) -> Option<P> {
        let index = self.ids.remove::<P>()?;
        self.pearls.remove(index).downcast::<P>()
    }

    pub fn drop(&mut self, id: &PearlId) -> bool {
        let Some(index) = self.ids.remove_id(&id) else { return false };
        self.pearls.remove(index);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPearl1;
    struct TestPearl2;
    struct TestPearl3;

    #[test]
    fn manipulation() {
        let mut set = PearlSet::new();
        assert!(set.len() == 0);
        assert!(set.is_empty());

        set.insert(TestPearl1);
        set.insert(TestPearl2);
        assert!(set.len() == 2);
        assert!(!set.is_empty());

        set.remove::<TestPearl1>();
        assert!(set.len() == 1);
        assert!(!set.is_empty());

        set.remove::<TestPearl1>();
        assert!(set.len() == 1);
        assert!(!set.is_empty());

        set.insert(TestPearl3);
        assert!(set.len() == 2);
        assert!(!set.is_empty());

        set.drop(&TestPearl2.pearl_id());
        assert!(set.len() == 1);
        assert!(!set.is_empty());

        set.drop(&PearlId::of::<TestPearl3>());
        assert!(set.len() == 0);
        assert!(set.is_empty());
    }

    #[test]
    fn id_comparison() {
        let mut set1 = PearlSet::new();
        set1.insert(TestPearl1);
        set1.insert(TestPearl2);
        set1.insert(TestPearl3);

        let mut set2 = PearlSet::new();
        set2.insert(TestPearl1);
        set2.insert(TestPearl2);

        assert!(set1.ids().contains::<TestPearl1>().is_some());
        assert!(set1.ids().contains::<TestPearl2>().is_some());
        assert!(set1.ids().contains::<TestPearl3>().is_some());
        assert!(set2.ids().contains::<TestPearl1>().is_some());
        assert!(set2.ids().contains::<TestPearl2>().is_some());
        assert!(set2.ids().contains::<TestPearl3>().is_none());
        assert!(set1.ids().contains_set(set2.ids()));
    }
}
