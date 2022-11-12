use boba_core::{storage::ControllerStorage, BobaStage};
use log::{error, warn};

use crate::TaroRenderer;

pub struct OnTaroRender;

impl BobaStage for OnTaroRender {
    type StageData = ();

    fn run(
        &mut self,
        controllers: &mut ControllerStorage<Self>,
        resources: &mut boba_core::BobaResources,
    ) {
        let Some(renderer) = resources.get::<TaroRenderer>() else {
            warn!("Skipping TaroRenderStage. No TaroRenderer found in resources.");
            return;
        };

        let Some(render_resources) = renderer.resources() else {
            warn!("Skipping TaroRenderStage. Found TaroRenderer but it is not initialized to a window.");
            return;
        };

        let output = match render_resources.surface.get_current_texture() {
            Ok(surface) => surface,
            Err(surface_error) => {
                error!(
                    "Skipping TaroRenderStage. Could not get current surface texture. Error: {:?}",
                    surface_error
                );
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            render_resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        controllers.update(&(), resources);

        // re-access renderer after passing resources to controllers
        // renderer could have been removed or changed, so this is necessary
        // to appease the borrow checker gods
        let Some(renderer) = resources
            .get_mut::<TaroRenderer>() else {
                warn!("Skipping TaroRenderStage. No TaroRenderer found in resources.");
                return;
            };

        renderer.execute_render_phases(&view, &mut encoder);

        let Some(render_resources) = renderer.resources() else {
                warn!("Cannot submit rendered frame to TaroRenderer. TaroRenderer is unitialized.");
                return;
            };

        render_resources
            .queue
            .submit(std::iter::once(encoder.finish()));

        output.present();
    }
}
