use boba_core::{BobaResources, BobaResult, BobaStage, PearlRegistry};

pub struct MilkTeaEvent<T> {
    data: T,
}

impl<T> MilkTeaEvent<T> {
    pub(crate) fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T: 'static> BobaStage for MilkTeaEvent<T> {
    type Data = T;

    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources) -> BobaResult {
        registry.run_stage::<MilkTeaEvent<T>>(&self.data, resources);
        Ok(())
    }
}
