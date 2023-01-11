mod event;

pub use event::*;

#[derive(Debug, Clone, Copy)]
pub struct MilkTeaSize {
    pub width: u32,
    pub height: u32,
}

impl MilkTeaSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
