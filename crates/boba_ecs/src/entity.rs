/// An entity is a link into specific components of a [`World`][crate::World]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Entity(u64);

impl Entity {
    const INDEX_BITS: u32 = u64::BITS / 2;
    const GEN_BITS: u32 = u64::BITS / 4;
    const META_BITS: u32 = u64::BITS / 4;
    const GEN_OFFSET: u32 = Self::INDEX_BITS;
    const META_OFFSET: u32 = Self::INDEX_BITS + Self::GEN_BITS;

    /// Returns a new entity with the raw `value`
    ///
    /// Marked as `unsafe`, as manually creating an entity
    /// could be used to index into a [`World`][crate::World] where this entity should be invalid
    #[inline]
    pub unsafe fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying `u64` used as an id for this entity
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }

    /// Returns a new entity containing the raw parts `index`, `gen`, and `meta`
    ///
    /// Marked as `unsafe`, as manually creating an entity
    /// could be used to index into a [`World`][crate::World] where this entity should be invalid
    #[inline]
    pub unsafe fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        Self(
            (index as u64)
                + ((gen as u64) << Self::GEN_OFFSET)
                + ((meta as u64) << Self::META_OFFSET),
        )
    }

    /// Decomposes this entity into its raw parts:
    /// - `u32`: index
    /// - `u16`: generation
    /// - `u16`: metadata
    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (
            self.0 as u32,
            (self.0 >> Self::GEN_BITS) as u16,
            (self.0 >> Self::META_BITS) as u16,
        )
    }

    /// Returns the raw index value for this entity
    #[inline]
    pub fn index(self) -> u32 {
        self.0 as u32
    }

    /// Returns the index value for this entity as a `usize`
    #[inline]
    pub fn uindex(self) -> usize {
        self.0 as u32 as usize
    }

    /// Returns the raw generation value for this entity
    #[inline]
    pub fn generation(self) -> u16 {
        (self.0 >> Self::GEN_BITS) as u16
    }

    /// Returns the raw metadata value for this entity
    #[inline]
    pub fn metadata(self) -> u16 {
        (self.0 >> Self::META_BITS) as u16
    }
}
