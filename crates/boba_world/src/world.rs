use crate::{NodeMap, PearlMap};

#[derive(Default)]
pub struct World<const ID: usize> {
    pub(crate) nodes: NodeMap<ID>,
    pub(crate) pearls: PearlMap<ID>,
}

impl<const ID: usize> World<ID> {
    pub fn new() -> Self {
        Self::default()
    }
}
