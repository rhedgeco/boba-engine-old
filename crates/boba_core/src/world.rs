use crate::{events::Event, pearls::PearlManager, BobaResources};

/// Central storage for [`PearlCollection`] and [`BobaResources`]
#[derive(Default)]
pub struct BobaWorld {
    pub pearls: PearlManager,
    pub resources: BobaResources,
}

impl BobaWorld {
    /// Returns a new world
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Triggers an `event` that will call every pearl stored in this world that is listening for that event.
    ///
    /// For a pearl to "listen" to an event it must implement [`EventListener`]
    /// and register that event in the `register` method of [`Pearl`].
    #[inline]
    pub fn trigger<E: Event>(&mut self, event: &E) {
        self.pearls.trigger(event, &mut self.resources);
    }
}
