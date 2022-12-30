use boba_core::{BobaResources, PearlRegistry, StageCollection};

pub trait MilkTeaPlugin {
    fn setup(
        registry: &mut PearlRegistry,
        startup_stages: &mut StageCollection,
        main_stages: &mut StageCollection,
        resources: &mut BobaResources,
    );
}
