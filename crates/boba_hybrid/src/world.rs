use crate::{pearls::PearlCollection, BobaResources};

#[derive(Default)]
pub struct World {
    pub pearls: PearlCollection,
    pub resources: BobaResources,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}
