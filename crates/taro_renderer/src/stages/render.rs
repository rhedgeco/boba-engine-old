use boba_core::{BobaResources, BobaResult, BobaStage, PearlRegistry};
use log::warn;

use crate::{TaroRenderPasses, TaroRenderPearls, TaroRenderer};

pub struct OnTaroRender;

impl BobaStage for OnTaroRender {
    type Data = ();

    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources) -> BobaResult {
        // run pearls that are listening for this stage
        registry.run_stage::<OnTaroRender>(&(), resources);

        let renderer = resources.get::<TaroRenderer>()?;
        let pearls = resources.get::<TaroRenderPearls>()?;
        let mut passes = resources.get_mut::<TaroRenderPasses>()?;
        if passes.len() == 0 {
            warn!("No render passes. Skipping render stage");
            return Ok(());
        }

        let output = renderer.surface().get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            renderer
                .hardware()
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        passes.render(&pearls, &view, &mut encoder, renderer.hardware());

        renderer
            .hardware()
            .queue
            .submit(std::iter::once(encoder.finish()));

        Ok(output.present())
    }
}
