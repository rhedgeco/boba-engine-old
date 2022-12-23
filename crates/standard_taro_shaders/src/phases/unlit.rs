use taro_renderer::{renderers::TaroMeshRenderer, wgpu, TaroRenderPhase};

use crate::shaders::UnlitShader;

pub struct UnlitRenderPhase;

impl TaroRenderPhase for UnlitRenderPhase {
    fn render(
        &mut self,
        view: &taro_renderer::wgpu::TextureView,
        camera_matrix: &taro_renderer::glam::Mat4,
        encoder: &mut taro_renderer::wgpu::CommandEncoder,
        pearls: &taro_renderer::RenderPearls,
        hardware: &taro_renderer::TaroHardware,
    ) {
        let mut renderers = pearls.collect::<TaroMeshRenderer<UnlitShader>>();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Unlit Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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

        for renderer in renderers.iter_mut() {
            renderer.render(&mut pass, camera_matrix, hardware);
        }
    }
}
