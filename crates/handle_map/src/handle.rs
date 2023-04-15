use std::{hash::Hash, marker::PhantomData};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RawHandle {
    pub id: u64,
}

impl RawHandle {
    const INDEX_BITS: u32 = u64::BITS / 2;
    const GEN_BITS: u32 = u64::BITS / 4;
    const META_BITS: u32 = u64::BITS / 4;
    const GEN_OFFSET: u32 = Self::INDEX_BITS;
    const META_OFFSET: u32 = Self::INDEX_BITS + Self::GEN_BITS;

    /// Returns a new handle containing the raw parts `index`, `gen`, and `meta`
    #[inline]
    pub fn from_raw_parts_usize(index: usize, gen: u16, meta: u16) -> Self {
        if index > u32::MAX as usize {
            panic!("RawHandle capacity overflow");
        }

        Self::from_raw_parts(index as u32, gen, meta)
    }

    /// Returns a new handle containing the raw parts `index`, `gen`, and `meta`
    #[inline]
    pub fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        let id = (index as u64)
            + ((gen as u64) << Self::GEN_OFFSET)
            + ((meta as u64) << Self::META_OFFSET);

        RawHandle { id }
    }

    /// Decomposes this handle into its raw parts:
    /// - `u32`: index
    /// - `u16`: generation
    /// - `u16`: metadata
    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (
            self.id as u32,
            (self.id >> Self::GEN_BITS) as u16,
            (self.id >> Self::META_BITS) as u16,
        )
    }

    pub fn increment_generation(self) -> Self {
        let (index, gen, meta) = self.into_raw_parts();
        Self::from_raw_parts(index, gen.wrapping_add(1), meta)
    }

    /// Transforms this raw handle into a typed handle
    pub fn into_type<T>(self) -> Handle<T> {
        Handle {
            raw: self,
            _type: PhantomData,
        }
    }

    /// Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the raw index value for this handle
    #[inline]
    pub fn index(&self) -> u32 {
        self.id as u32
    }

    /// Returns the index value for this handle as a `usize`
    #[inline]
    pub fn uindex(&self) -> usize {
        self.id as u32 as usize
    }

    /// Returns the raw generation value for this handle
    #[inline]
    pub fn generation(&self) -> u16 {
        (self.id >> Self::GEN_BITS) as u16
    }

    /// Returns the raw metadata value for this handle
    #[inline]
    pub fn metadata(&self) -> u16 {
        (self.id >> Self::META_BITS) as u16
    }
}

pub struct Handle<T> {
    raw: RawHandle,
    _type: PhantomData<*const T>,
}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
        self._type.hash(state);
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> Eq for Handle<T> {}
impl<T> PartialEq for Handle<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Handle<T> {
    /// Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn into_raw(self) -> RawHandle {
        self.raw
    }

    /// Returns a new handle containing the raw parts `index`, `gen`, and `meta`
    #[inline]
    pub fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        RawHandle::from_raw_parts(index, gen, meta).into_type()
    }

    /// Decomposes this handle into its raw parts:
    /// - `u32`: index
    /// - `u16`: generation
    /// - `u16`: metadata
    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        self.raw.into_raw_parts()
    }

    /// Transforms this handles type into another type
    #[inline]
    pub fn into_type<U>(self) -> Handle<U> {
        self.raw.into_type()
    }

    /// Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn id(&self) -> u64 {
        self.raw.id()
    }

    /// Returns the raw index value for this handle
    #[inline]
    pub fn index(&self) -> u32 {
        self.raw.index()
    }

    /// Returns the index value for this handle as a `usize`
    #[inline]
    pub fn uindex(&self) -> usize {
        self.raw.uindex()
    }

    /// Returns the raw generation value for this handle
    #[inline]
    pub fn generation(&self) -> u16 {
        self.raw.generation()
    }

    /// Returns the raw metadata value for this handle
    #[inline]
    pub fn metadata(&self) -> u16 {
        self.raw.metadata()
    }
}
