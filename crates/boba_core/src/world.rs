use crate::{pearls::PearlCollection, BobaResources};

/// Central storage for [`PearlCollection`] and [`BobaResources`]
#[derive(Default)]
pub struct World {
    pub pearls: PearlCollection,
    pub resources: BobaResources,
}

impl World {
    /// Returns a new world
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}
