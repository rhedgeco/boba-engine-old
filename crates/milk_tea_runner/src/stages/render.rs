use boba_core::BobaStage;
use wgpu::RenderPass;

use crate::MilkTeaRender;

pub struct MilkTeaRenderStage;

impl BobaStage for MilkTeaRenderStage {
    type StageData<'a> = RenderPass<'a>;

    fn run(
        &mut self,
        controllers: &mut boba_core::controller_storage::ControllerStorage,
        resources: &mut boba_core::BobaResources,
    ) {
        let renderer = resources
            .get::<MilkTeaRender>()
            .expect("Renderer not found in resources");

        let Ok(output) = renderer.surface().get_current_texture() else {
                return;
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

        controllers.update::<MilkTeaRenderStage>(&mut render_pass, resources);

        // drop and re-access renderer to appease borrow gods
        drop(render_pass);
        let renderer = resources
            .get::<MilkTeaRender>()
            .expect("Renderer not found in resources");

        renderer.queue().submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
