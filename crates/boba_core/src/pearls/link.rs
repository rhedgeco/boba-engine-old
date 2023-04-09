use std::ops::{Deref, DerefMut};

use handle_map::{Handle, RawHandle};

use super::Pearl;

/// Represents a link to a single pearl in a [`PearlCollection`].
pub struct Link<P: Pearl> {
    pub(crate) map: RawHandle,
    pub(crate) pearl: Handle<P>,
}

impl<P: Pearl> Copy for Link<P> {}

impl<P: Pearl> Clone for Link<P> {
    fn clone(&self) -> Self {
        Self {
            map: self.map,
            pearl: self.pearl,
        }
    }
}

impl<P: Pearl> Link<P> {
    /// Returns a new link with `map` and `pearl`
    pub(crate) fn new(map: RawHandle, pearl: Handle<P>) -> Self {
        Self { map, pearl }
    }
}

pub struct PearlLink<'a, P: Pearl> {
    pearl: &'a mut P,
    link: Link<P>,
}

impl<'a, P: Pearl> PearlLink<'a, P> {
    pub fn new(pearl: &'a mut P, link: Link<P>) -> Self {
        Self { pearl, link }
    }

    pub fn link(&self) -> &Link<P> {
        &self.link
    }
}

impl<'a, P: Pearl> Deref for PearlLink<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl
    }
}

impl<'a, P: Pearl> DerefMut for PearlLink<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pearl
    }
}
