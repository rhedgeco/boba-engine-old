use boba_core::{BobaResources, BobaStage, PearlRegistry};

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

pub struct OnMilkTeaResize {
    pub size: MilkTeaSize,
}

impl OnMilkTeaResize {
    pub fn new(size: MilkTeaSize) -> Self {
        Self { size }
    }
}

impl BobaStage for OnMilkTeaResize {
    type Data = MilkTeaSize;

    fn run(
        &mut self,
        registry: &mut PearlRegistry,
        resources: &mut BobaResources,
    ) -> boba_core::BobaResult {
        registry.run_stage::<Self>(&self.size, resources);
        Ok(())
    }
}
