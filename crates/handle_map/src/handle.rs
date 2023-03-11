use std::{marker::PhantomData, ops::Deref};

/// An untyped handle into some map
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyHandle(u64);

impl AnyHandle {
    /// Returns the index of this handle
    #[inline]
    pub const fn index(&self) -> u32 {
        self.0 as u32
    }

    /// Returns the index of this handle as `usize`
    #[inline]
    pub const fn uindex(&self) -> usize {
        self.index() as usize
    }

    /// Returns the generation of this handle
    #[inline]
    pub const fn generation(&self) -> u16 {
        (self.0 >> 32) as u16
    }

    /// Returns the metadata of this handle
    #[inline]
    pub const fn meta(&self) -> u16 {
        (self.0 >> 48) as u16
    }

    /// Constructs a new handle out of its raw parts
    #[inline]
    pub const fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        Self((index as u64) + ((gen as u64) << 32) + ((meta as u64) << 48))
    }

    /// Decomposes this handle into its raw parts
    #[inline]
    pub const fn into_raw_parts(self) -> (u32, u16, u16) {
        (self.0 as u32, (self.0 >> 32) as u16, (self.0 >> 48) as u16)
    }

    /// Constructs a new handle out of a raw id
    #[inline]
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Decomposes this handle into its raw id
    #[inline]
    pub const fn into_raw(self) -> u64 {
        self.0
    }

    /// Modifies the handle in place using `f`
    #[inline]
    pub fn modify(&mut self, f: impl FnOnce(u32, u16, u16) -> (u32, u16, u16)) {
        let (index, gen, meta) = self.into_raw_parts();
        let (index, gen, meta) = f(index, gen, meta);
        self.0 = (index as u64) + ((gen as u64) << 32) + ((meta as u64) << 48)
    }
}

/// A handle to some map of type `T`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle<T> {
    handle: AnyHandle,
    _type: PhantomData<*const T>,
}

impl<T> Handle<T> {
    /// Constructs a new handle out of its raw parts
    #[inline]
    pub const fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        Self {
            handle: AnyHandle::from_raw_parts(index, gen, meta),
            _type: PhantomData,
        }
    }

    /// Constructs a new handle out of a raw id
    #[inline]
    pub const fn from_raw(value: u64) -> Self {
        Self {
            handle: AnyHandle::from_raw(value),
            _type: PhantomData,
        }
    }

    /// Returns the untyped version of this handle
    #[inline]
    pub const fn any(&self) -> AnyHandle {
        self.handle
    }
}

impl<T> Into<AnyHandle> for Handle<T> {
    fn into(self) -> AnyHandle {
        self.handle
    }
}

impl<T> Deref for Handle<T> {
    type Target = AnyHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
