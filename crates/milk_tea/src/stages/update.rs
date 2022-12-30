use std::time::Instant;

use boba_core::{BobaResources, BobaResult, BobaStage, PearlRegistry};

pub struct MilkTeaUpdate {
    instant: Option<Instant>,
}

impl Default for MilkTeaUpdate {
    fn default() -> Self {
        Self { instant: None }
    }
}

impl BobaStage for MilkTeaUpdate {
    type Data = f32;

    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources) -> BobaResult {
        let delta = match self.instant {
            Some(instant) => instant.elapsed().as_secs_f32(),
            None => 0f32,
        };

        self.instant = Some(Instant::now());

        registry.run_stage::<MilkTeaUpdate>(&delta, resources);

        Ok(())
    }
}
