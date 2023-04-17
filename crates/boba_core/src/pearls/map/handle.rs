use std::{marker::PhantomData, ops::Deref};

use crate::pearls::Pearl;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RawHandle {
    id: u64,
}

impl RawHandle {
    const MAP_OFFSET: u32 = u32::BITS;
    const GEN_OFFSET: u32 = u32::BITS + u16::BITS;

    #[inline]
    pub fn from_raw_value(id: u64) -> Self {
        Self { id }
    }

    #[inline]
    pub fn from_raw_parts(index: u32, map_id: u16, generation: u16) -> Self {
        Self {
            id: (index as u64)
                + ((map_id as u64) << Self::MAP_OFFSET)
                + ((generation as u64) << Self::GEN_OFFSET),
        }
    }

    #[inline]
    pub fn from_raw_usize_parts(index: usize, map_id: usize, generation: u16) -> Self {
        if index > u32::MAX as usize {
            panic!("Handle index overflow");
        }

        if map_id > u16::MAX as usize {
            panic!("Handle map_id overflow");
        }

        Self::from_raw_parts(index as u32, map_id as u16, generation)
    }

    #[inline]
    pub fn into_raw_value(self) -> u64 {
        self.id
    }

    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (self.index(), self.map_id(), self.generation())
    }

    #[inline]
    pub fn into_raw_usize_parts(self) -> (usize, usize, u16) {
        (self.uindex(), self.umap_id(), self.generation())
    }

    #[inline]
    pub fn into_type<P: Pearl>(self) -> Handle<P> {
        Handle {
            raw: self,
            _type: PhantomData,
        }
    }

    #[inline]
    pub fn index(self) -> u32 {
        self.id as u32
    }

    #[inline]
    pub fn uindex(self) -> usize {
        self.index() as usize
    }

    #[inline]
    pub fn map_id(self) -> u16 {
        (self.id >> Self::MAP_OFFSET) as u16
    }

    #[inline]
    pub fn umap_id(self) -> usize {
        self.map_id() as usize
    }

    #[inline]
    pub fn generation(self) -> u16 {
        (self.id >> Self::GEN_OFFSET) as u16
    }

    #[inline]
    pub fn increment_generation(self) -> Self {
        let (index, map, gen) = self.into_raw_parts();
        Self::from_raw_parts(index, map, gen.wrapping_add(1))
    }
}

pub struct Handle<P: Pearl> {
    raw: RawHandle,
    _type: PhantomData<*const P>,
}

impl<P: Pearl> Handle<P> {
    #[inline]
    pub fn from_raw_value(id: u64) -> Self {
        RawHandle::from_raw_value(id).into_type()
    }

    #[inline]
    pub fn from_raw_parts(index: u32, map_id: u16, generation: u16) -> Self {
        RawHandle::from_raw_parts(index, map_id, generation).into_type()
    }

    #[inline]
    pub fn from_raw_usize_parts(index: usize, map_id: usize, generation: u16) -> Self {
        RawHandle::from_raw_usize_parts(index, map_id, generation).into_type()
    }

    #[inline]
    pub fn into_raw(self) -> RawHandle {
        self.raw
    }
}

impl<P: Pearl> Deref for Handle<P> {
    type Target = RawHandle;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
