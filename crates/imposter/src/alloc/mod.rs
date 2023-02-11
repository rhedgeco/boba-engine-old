mod global;

use std::alloc::Layout;

pub use global::*;

pub trait MemoryBuilder {
    fn new<T: 'static>() -> Self;
    fn from_layout(layout: Layout) -> Self;
    fn ptr(&self) -> *mut u8;
    fn capacity(&self) -> usize;
    fn layout(&self) -> Layout;
    fn resize(&mut self, len: usize);
}
