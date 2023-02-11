use std::{any::TypeId, mem, ptr};

use crate::{
    alloc::{GlobalMemoryBuilder, MemoryBuilder},
    Imposter, ImposterDrop,
};

pub struct ImposterVec<M: MemoryBuilder = GlobalMemoryBuilder> {
    typeid: TypeId,
    memory: M,
    len: usize,
    drop: Option<ImposterDrop>,
}

impl<M: MemoryBuilder> Drop for ImposterVec<M> {
    #[inline]
    fn drop(&mut self) {
        self.clear()
    }
}

impl<M: MemoryBuilder> ImposterVec<M> {
    /// Creates a new `ImposterVec` that can hold items of type `T`
    pub fn new<T: 'static>() -> Self {
        Self {
            typeid: TypeId::of::<T>(),
            memory: M::new::<T>(),
            len: 0,
            drop: match mem::needs_drop::<T>() {
                false => None,
                true => Some(Imposter::drop_impl::<T>),
            },
        }
    }

    /// Creates a new `ImposterVec` with the initial value `imposter`
    pub fn from_imposter(imposter: Imposter) -> Self {
        let mut vec = Self {
            typeid: imposter.type_id(),
            memory: M::from_layout(imposter.layout()),
            len: 0,
            drop: imposter.drop_fn(),
        };
        vec.push_imposter(imposter);
        vec
    }

    /// Appends an [`Imposter`] to the end of the vector, returning `None`.
    ///
    /// If the imposter is not valid for this vec, it will be returned as `Some(Imposter)`
    pub fn push_imposter(&mut self, imposter: Imposter) -> Option<Imposter> {
        if imposter.type_id() != self.typeid {
            return Some(imposter);
        }

        if self.len == self.memory.capacity() {
            self.memory.resize(self.memory.capacity() * 2);
        }

        let data_size = self.memory.layout().size();
        let offset = self.len * data_size;
        unsafe {
            let end = self.memory.ptr().add(offset);
            ptr::copy_nonoverlapping(imposter.data().as_ptr(), end, data_size);
        }
        self.len += 1;

        // forget the imposter so it doesnt call its destructor
        mem::forget(imposter);

        None
    }

    /// Appends `item` to the end of the vector, returning `None`.
    ///
    /// If the item is not valid for this vec, it will be returned as `Some(T)`
    pub fn push_item<T: 'static>(&mut self, item: T) -> Option<T> {
        if TypeId::of::<T>() != self.typeid {
            return Some(item);
        }

        if self.len == self.memory.capacity() {
            self.memory.resize(self.memory.capacity() * 2);
        }

        let item_ptr = ptr::NonNull::from(&item).cast::<u8>().as_ptr();
        let data_size = self.memory.layout().size();
        let offset = self.len * data_size;
        unsafe {
            let end = self.memory.ptr().add(offset);
            ptr::copy_nonoverlapping(item_ptr, end, data_size);
        }
        mem::forget(item);
        self.len += 1;

        None
    }

    /// Drops the value at `index` by swapping it with the last value
    pub fn swap_drop(&mut self, index: usize) {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        let data_size = self.memory.layout().size();
        let last_offset = (self.len - 1) * data_size;
        let drop_offset = index * data_size;
        unsafe {
            let last = self.memory.ptr().add(last_offset);
            let drop = self.memory.ptr().add(drop_offset);
            ptr::swap_nonoverlapping(last, drop, data_size);
            if let Some(drop) = self.drop {
                drop(last)
            }
        }
        self.len -= 1;
    }

    /// Clears all the elements in the vector, calling their drop function if necessary
    pub fn clear(&mut self) {
        let len = self.len;
        self.len = 0;

        if let Some(drop) = self.drop {
            let mut ptr = self.memory.ptr();
            let data_size = self.memory.layout().size();
            for _ in 0..len {
                unsafe {
                    drop(ptr);
                    ptr = ptr.add(data_size)
                };
            }
        }
    }
}
