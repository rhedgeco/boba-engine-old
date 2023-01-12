use std::time::Instant;

use boba_core::{BobaResult, BobaStage};

use crate::RapierPhysics;

#[derive(Default)]
pub struct OnRapierUpdate {
    time_collector: f32,
    instant: Option<Instant>,
}

impl BobaStage for OnRapierUpdate {
    type Data = ();

    fn run(
        &mut self,
        registry: &mut boba_core::PearlRegistry,
        resources: &mut boba_core::BobaResources,
    ) -> BobaResult {
        self.time_collector += match &self.instant {
            Some(instant) => instant.elapsed().as_secs_f32(),
            None => 0.,
        };

        if self.time_collector > (1. / 50.) {
            resources.get_mut::<RapierPhysics>()?.step();
            registry.run_stage::<OnRapierUpdate>(&(), resources);
            self.time_collector = 0.;
        }

        self.instant = Some(Instant::now());
        Ok(())
    }
}
