use std::slice::IterMut;

use crate::pearls::Pearl;

use super::{Handle, PearlAccessMap, PearlData, RawHandle};

pub struct ExclusiveStream<'a, P: Pearl> {
    iterator: IterMut<'a, PearlData<P>>,
    access: &'a mut PearlAccessMap<'a>,
}

impl<'a, P: Pearl> ExclusiveStream<'a, P> {
    pub(super) fn new(access: &'a mut PearlAccessMap<'a>) -> Option<Self> {
        let access_ptr = access as *mut PearlAccessMap;
        Some(Self {
            iterator: access.iter_mut()?,
            // SAFETY: This is unsafe because both the iterator and access variable alias over the same data.
            // However, since the data is returned through an ExclusivePearlAccess,
            // we restrict access to the only data that the iterator is currently handing out access to.
            // And since this isn't technically an iterator, but a streamer,
            // the data from the data must go out of scope before 'next' can be called again.
            // Thus, there is never an instance where multiple mutable access is exposed to the user.
            access: unsafe { &mut *access_ptr },
        })
    }

    pub fn next<'access>(
        &'access mut self,
    ) -> Option<(&'a mut PearlData<P>, ExclusivePearlAccess<'a, 'access>)> {
        let pearl_data = self.iterator.next()?;
        let exclusive = ExclusivePearlAccess {
            exclude: pearl_data.handle().into_raw(),
            access: self.access,
        };
        Some((pearl_data, exclusive))
    }
}

pub struct ExclusivePearlAccess<'a, 'access> {
    exclude: RawHandle,
    access: &'access mut PearlAccessMap<'a>,
}

impl<'a, 'access> ExclusivePearlAccess<'a, 'access> {
    pub fn get_excluded_handle(access: &ExclusivePearlAccess) -> RawHandle {
        access.exclude
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        if handle == self.exclude {
            return None;
        }

        self.access.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        if handle == self.exclude {
            return None;
        }

        self.access.get_mut(handle)
    }
}
