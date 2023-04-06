use std::{hash::Hash, marker::PhantomData};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RawHandle {
    id: u64,
}

impl RawHandle {
    /// Converts this raw handle into a [`Handle`] of for type `T`
    pub fn into_type<T>(self) -> Handle<T> {
        Handle {
            raw: self,
            _type: PhantomData,
        }
    }

    /// Converts this raw handle reference into a [`Handle`] reference of for type `T`
    pub fn as_type<T>(&self) -> &Handle<T> {
        unsafe { std::mem::transmute(self) }
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
    const INDEX_BITS: u32 = u64::BITS / 2;
    const GEN_BITS: u32 = u64::BITS / 4;
    const META_BITS: u32 = u64::BITS / 4;
    const GEN_OFFSET: u32 = Self::INDEX_BITS;
    const META_OFFSET: u32 = Self::INDEX_BITS + Self::GEN_BITS;

    /// Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn into_raw(self) -> RawHandle {
        self.raw
    }

    /// Returns a new handle containing the raw parts `index`, `gen`, and `meta`
    #[inline]
    pub fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        let id = (index as u64)
            + ((gen as u64) << Self::GEN_OFFSET)
            + ((meta as u64) << Self::META_OFFSET);

        RawHandle { id }.into_type()
    }

    /// Decomposes this handle into its raw parts:
    /// - `u32`: index
    /// - `u16`: generation
    /// - `u16`: metadata
    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (
            self.raw.id as u32,
            (self.raw.id >> Self::GEN_BITS) as u16,
            (self.raw.id >> Self::META_BITS) as u16,
        )
    }

    /// Consumes self and transforms it into a handle for another type
    ///
    /// # Warning
    /// While this is not unsafe and will not cause undefined behavior on its own,
    /// it may not behave as expected. A handle should usually be used on the map it is associated with.
    #[inline]
    pub fn into_type<U>(self) -> Handle<U> {
        Handle {
            raw: self.raw,
            _type: PhantomData,
        }
    }

    /// Transforms a handle reference into another types handle
    ///
    /// # Warning
    /// While this is not unsafe and will not cause undefined behavior on its own,
    /// it may not behave as expected. A handle should usually be used on the map it is associated with.
    #[inline]
    pub fn as_type<U>(&self) -> &Handle<U> {
        unsafe { std::mem::transmute(self) }
    }

    // Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn id(&self) -> u64 {
        self.raw.id
    }

    /// Returns the raw index value for this handle
    #[inline]
    pub fn index(&self) -> u32 {
        self.raw.id as u32
    }

    /// Returns the index value for this handle as a `usize`
    #[inline]
    pub fn uindex(&self) -> usize {
        self.raw.id as u32 as usize
    }

    /// Returns the raw generation value for this handle
    #[inline]
    pub fn generation(&self) -> u16 {
        (self.raw.id >> Self::GEN_BITS) as u16
    }

    /// Returns the raw metadata value for this handle
    #[inline]
    pub fn metadata(&self) -> u16 {
        (self.raw.id >> Self::META_BITS) as u16
    }
}
