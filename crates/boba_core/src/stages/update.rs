use std::time::Instant;

use crate::{BobaResources, BobaResult, BobaStage, PearlRegistry};

pub struct BobaUpdate {
    instant: Option<Instant>,
}

impl Default for BobaUpdate {
    fn default() -> Self {
        Self { instant: None }
    }
}

impl BobaStage for BobaUpdate {
    type Data = f32;

    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources) -> BobaResult {
        let delta = match self.instant {
            Some(instant) => instant.elapsed().as_secs_f32(),
            None => 0f32,
        };

        self.instant = Some(Instant::now());

        registry.run_stage::<BobaUpdate>(&delta, resources);

        Ok(())
    }
}
