use std::{
    alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout},
    ptr::NonNull,
};

use super::MemoryBuilder;

pub struct GlobalMemoryBuilder {
    ptr: NonNull<u8>,
    cap: usize,
    layout: Layout,
}

impl GlobalMemoryBuilder {
    /// Creates a dangling pointer with a specified layout
    ///
    /// # Safety
    /// This pointer is dangling and invalid.
    /// This is not inherently unsafe, unless the pointer is dereferenced.
    /// This pointer should only be used to `alloc` new memory with the same alignment.
    #[inline]
    unsafe fn dangling_ptr(layout: &Layout) -> NonNull<u8> {
        NonNull::new_unchecked(layout.align() as *mut u8)
    }
}

impl MemoryBuilder for GlobalMemoryBuilder {
    fn new<T: 'static>() -> Self {
        Self {
            ptr: NonNull::<T>::dangling().cast(),
            cap: 0,
            layout: Layout::new::<T>(),
        }
    }

    fn from_layout(layout: Layout) -> Self {
        // SAFETY:
        // Creates dangling pointer, which is not inherently unsafe.
        // It is only unsafe if is dereferenced.
        // This pointer, in its dangling state, should only be used to alloc new memory locations
        let ptr = unsafe { Self::dangling_ptr(&layout) };
        Self {
            ptr,
            cap: 1,
            layout,
        }
    }

    #[inline]
    fn ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.cap
    }

    #[inline]
    fn layout(&self) -> Layout {
        self.layout
    }

    fn resize(&mut self, new_capacity: usize) {
        if self.cap == new_capacity {
            return;
        }

        if self.layout.size() != 0 {
            unsafe {
                let array_layout = Layout::from_size_align_unchecked(
                    self.layout.size() * self.cap,
                    self.layout.align(),
                );

                self.ptr = if new_capacity == 0 {
                    dealloc(self.ptr(), array_layout);
                    Self::dangling_ptr(&self.layout)
                } else {
                    let new_array_size = self.layout.size().checked_mul(new_capacity).unwrap();
                    let new_array_layout =
                        Layout::from_size_align_unchecked(new_array_size, self.layout.align());

                    if self.cap == 0 {
                        NonNull::new(alloc(new_array_layout))
                    } else {
                        NonNull::new(realloc(self.ptr(), array_layout, new_array_size))
                    }
                    .unwrap_or_else(|| handle_alloc_error(new_array_layout))
                }
            }
        }
    }
}
