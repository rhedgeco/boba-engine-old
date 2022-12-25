use crate::{BobaResources, PearlRegistry, StageCollection};

#[derive(Default)]
pub struct BobaApp {
    pub registry: PearlRegistry,
    pub startup_stages: StageCollection,
    pub main_stages: StageCollection,
    pub resources: BobaResources,
}
