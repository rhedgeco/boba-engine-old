use std::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::pearls::Pearl;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RawHandle {
    id: u64,
}

impl<P: Pearl> PartialEq<Handle<P>> for RawHandle {
    fn eq(&self, other: &Handle<P>) -> bool {
        &other.raw == self
    }
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
        let index = u32::try_from(index).expect("Handle index overflow");
        let map_id = u16::try_from(map_id).expect("Handle map_id overflow");
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
    pub fn increment_generation(&mut self) {
        let (index, map, gen) = self.into_raw_parts();
        *self = Self::from_raw_parts(index, map, gen.wrapping_add(1))
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

impl<P: Pearl> Copy for Handle<P> {}
impl<P: Pearl> Clone for Handle<P> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            _type: self._type.clone(),
        }
    }
}

impl<P: Pearl> PartialEq<RawHandle> for Handle<P> {
    fn eq(&self, other: &RawHandle) -> bool {
        other == &self.raw
    }
}

impl<P: Pearl> Eq for Handle<P> {}
impl<P: Pearl> PartialEq for Handle<P> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<P: Pearl> Hash for Handle<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<P: Pearl> Debug for Handle<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("raw", &self.raw)
            .field("_type", &self._type)
            .finish()
    }
}

impl<P: Pearl> DerefMut for Handle<P> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl<P: Pearl> Deref for Handle<P> {
    type Target = RawHandle;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

pub struct PearlData<P: Pearl> {
    pearl: P,
    handle: Handle<P>,
}

impl<P: Pearl> PearlData<P> {
    pub(super) fn new(pearl: P, handle: Handle<P>) -> Self {
        Self { pearl, handle }
    }

    pub fn into_data(self) -> (P, Handle<P>) {
        (self.pearl, self.handle)
    }

    pub fn handle(&self) -> Handle<P> {
        self.handle
    }
}

impl<P: Pearl> DerefMut for PearlData<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pearl
    }
}

impl<P: Pearl> Deref for PearlData<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.pearl
    }
}
