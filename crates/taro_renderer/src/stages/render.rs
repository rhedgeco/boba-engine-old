use boba_core::{storage::ControllerStorage, BobaStage};
use log::{error, warn};
use wgpu::RenderPass;

use crate::TaroRenderer;

pub struct TaroRenderStage;

impl BobaStage for TaroRenderStage {
    type StageData<'a> = RenderPass<'a>;

    fn run(
        &mut self,
        controllers: &mut ControllerStorage<Self>,
        resources: &mut boba_core::BobaResources,
    ) {
        let Some(renderer) = resources
            .get::<TaroRenderer>() else {
                warn!("Skipping TaroRenderStage. No TaroRenderer found.");
                return;
            };

        let output = match renderer.surface().get_current_texture() {
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
            renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        controllers.update(&mut render_pass, resources);
        drop(render_pass);

        // re-access renderer after passing resources to controllers
        // renderer could have been removed or changed
        let Some(renderer) = resources
            .get::<TaroRenderer>() else {
                warn!("Cannot submit rendered frame to TaroRenderer. No TaroRenderer found.");
                return;
            };

        renderer.queue().submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
