use std::marker::PhantomData;

#[derive(Copy, Eq, Debug, Hash)]
pub struct Handle<T> {
    id: u64,
    _type: PhantomData<*const T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Handle<T> {
    const INDEX_BITS: u32 = u64::BITS / 2;
    const GEN_BITS: u32 = u64::BITS / 4;
    const META_BITS: u32 = u64::BITS / 4;
    const GEN_OFFSET: u32 = Self::INDEX_BITS;
    const META_OFFSET: u32 = Self::INDEX_BITS + Self::GEN_BITS;

    /// Returns a new handle with the raw `id`
    #[inline]
    pub fn from_raw(id: u64) -> Self {
        Self {
            id,
            _type: PhantomData,
        }
    }

    /// Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.id
    }

    /// Returns a new handle containing the raw parts `index`, `gen`, and `meta`
    #[inline]
    pub fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        let id = (index as u64)
            + ((gen as u64) << Self::GEN_OFFSET)
            + ((meta as u64) << Self::META_OFFSET);

        Self::from_raw(id)
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

    // Returns the underlying `u64` used as an id for this handle
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
