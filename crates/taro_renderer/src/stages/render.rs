use boba_core::{storage::ControllerStorage, BobaStage};
use log::{error, warn};

use crate::{renderers::TaroMeshRenderer, types::TaroCompiler, TaroRenderer};

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
        // renderer could have been removed or changed
        let Some(renderer) = resources
            .get_mut::<TaroRenderer>() else {
                warn!("Skipping TaroRenderStage. No TaroRenderer found in resources.");
                return;
            };

        // TODO: MAKE THIS HARDCODED PASS INTO A PROGRAMMABLE STAGE
        {
            let meshes = renderer.render_controllers().collect::<TaroMeshRenderer>();
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

            for mesh in meshes.iter() {
                let pipeline = mesh.pipeline().as_ref().unwrap();
                let buffers = mesh.mesh().get_data().as_ref().unwrap();

                render_pass.set_pipeline(&pipeline.render_pipeline);
                render_pass.set_bind_group(0, &pipeline.bind_group, &[]);
                render_pass.set_vertex_buffer(0, buffers.vertex_buffer.raw_buffer().slice(..));
                render_pass.set_index_buffer(
                    buffers.index_buffer.raw_buffer().slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..buffers.index_buffer.buffer_length(), 0, 0..1);
            }
        }

        renderer.execute_render_phases(&mut encoder);

        // re-access renderer after passing resources to controllers
        // renderer could have been removed or changed
        let Some(renderer) = resources
            .get_mut::<TaroRenderer>() else {
                warn!("Cannot submit rendered frame to TaroRenderer. No TaroRenderer found.");
                return;
            };

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
