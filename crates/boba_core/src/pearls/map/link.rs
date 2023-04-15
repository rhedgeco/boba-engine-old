use std::ops::{Deref, DerefMut};

use handle_map::Handle;

use crate::pearls::Pearl;

pub struct PearlLink<P: Pearl> {
    pub(super) map_index: usize,
    pub(super) handle: Handle<P>,
}

impl<P: Pearl> PearlLink<P> {
    pub(super) fn new(map_index: usize, handle: Handle<P>) -> Self {
        Self { map_index, handle }
    }
}

impl<P: Pearl> Copy for PearlLink<P> {}
impl<P: Pearl> Clone for PearlLink<P> {
    fn clone(&self) -> Self {
        Self {
            map_index: self.map_index.clone(),
            handle: self.handle.clone(),
        }
    }
}

pub struct PearlRef<'a, P: Pearl> {
    pub(super) pearl: &'a P,
    pub(super) link: PearlLink<P>,
}

impl<'a, P: Pearl> PearlRef<'a, P> {
    pub(super) fn new(pearl: &'a P, link: PearlLink<P>) -> Self {
        Self { pearl, link }
    }

    pub fn link(&self) -> PearlLink<P> {
        self.link
    }
}

impl<'a, P: Pearl> Deref for PearlRef<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.pearl
    }
}

pub struct PearlMut<'a, P: Pearl> {
    pub(super) pearl: &'a mut P,
    pub(super) link: PearlLink<P>,
}

impl<'a, P: Pearl> PearlMut<'a, P> {
    pub(super) fn new(pearl: &'a mut P, link: PearlLink<P>) -> Self {
        Self { pearl, link }
    }

    pub fn link(&self) -> PearlLink<P> {
        self.link
    }
}

impl<'a, P: Pearl> DerefMut for PearlMut<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pearl
    }
}

impl<'a, P: Pearl> Deref for PearlMut<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.pearl
    }
}
