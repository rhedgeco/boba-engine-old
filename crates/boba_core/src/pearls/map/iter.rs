use std::slice;

use handle_map::{Handle, RawHandle};

use crate::pearls::Pearl;

use super::{PearlAccessMap, PearlLink, PearlMut, PearlRef};

pub struct Iter<'a, P: Pearl> {
    pub(super) map_index: usize,
    pub(super) inner: slice::Iter<'a, (Handle<P>, P)>,
}

impl<'a, P: Pearl> Iterator for Iter<'a, P> {
    type Item = PearlRef<'a, P>;

    fn next(&mut self) -> Option<Self::Item> {
        let (handle, pearl) = self.inner.next()?;
        let link = PearlLink::new(self.map_index, *handle);
        Some(PearlRef::new(pearl, link))
    }
}

pub struct IterMut<'a, P: Pearl> {
    pub(super) map_index: usize,
    pub(super) inner: slice::IterMut<'a, (Handle<P>, P)>,
}

impl<'a, P: Pearl> Iterator for IterMut<'a, P> {
    type Item = PearlMut<'a, P>;

    fn next(&mut self) -> Option<Self::Item> {
        let (handle, pearl) = self.inner.next()?;
        let link = PearlLink::new(self.map_index, *handle);
        Some(PearlMut::new(pearl, link))
    }
}

pub struct ExclusiveStream<'a, P: Pearl> {
    iterator: IterMut<'a, P>,
    access: &'a mut PearlAccessMap<'a>,
}

impl<'a, P: Pearl> ExclusiveStream<'a, P> {
    pub(super) fn new(access: &'a mut PearlAccessMap<'a>) -> Option<Self> {
        let access_ptr = access as *mut PearlAccessMap;
        Some(Self {
            iterator: access.iter_mut()?,
            access: unsafe { &mut *access_ptr },
        })
    }

    pub fn next<'access>(
        &'access mut self,
    ) -> Option<(PearlMut<P>, ExclusivePearlAccess<'a, 'access>)> {
        let pearl_mut = self.iterator.next()?;
        let exclusive = ExclusivePearlAccess {
            exclude: pearl_mut.link.handle.into_raw(),
            access: self.access,
        };
        Some((pearl_mut, exclusive))
    }
}

pub struct ExclusivePearlAccess<'a, 'access> {
    exclude: RawHandle,
    access: &'access mut PearlAccessMap<'a>,
}

impl<'a, 'access> ExclusivePearlAccess<'a, 'access> {
    pub fn get<P: Pearl>(&self, link: PearlLink<P>) -> Option<&P> {
        if link.handle.into_raw() == self.exclude {
            return None;
        }

        self.access.get(link)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: PearlLink<P>) -> Option<&mut P> {
        if link.handle.into_raw() == self.exclude {
            return None;
        }

        self.access.get_mut(link)
    }
}
