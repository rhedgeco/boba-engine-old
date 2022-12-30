use boba_core::{PearlCollector, ResourceCollector, StageCollector};

pub trait MilkTeaPlugin {
    fn setup(
        registry: &mut impl PearlCollector,
        startup_stages: &mut impl StageCollector,
        main_stages: &mut impl StageCollector,
        resources: &mut impl ResourceCollector,
    );
}
