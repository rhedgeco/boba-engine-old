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
    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        if handle.into_raw() == self.exclude {
            return None;
        }

        self.access.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        if handle.into_raw() == self.exclude {
            return None;
        }

        self.access.get_mut(handle)
    }
}
