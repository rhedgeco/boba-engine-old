use winit::dpi::PhysicalSize;

pub struct MilkTeaResize {
    size: PhysicalSize<u32>,
}

impl MilkTeaResize {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self { size }
    }

    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
}
