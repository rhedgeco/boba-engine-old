use taro_renderer::{
    pearls::TaroMeshRenderer, shading::bindings::CameraMatrix, wgpu, TaroRenderPass,
};

use crate::UnlitShader;

pub struct UnlitRenderPass;

impl TaroRenderPass for UnlitRenderPass {
    fn render(
        &mut self,
        pearls: &taro_renderer::TaroRenderPearls,
        camera_matrix: &CameraMatrix,
        view: &taro_renderer::wgpu::TextureView,
        encoder: &mut taro_renderer::wgpu::CommandEncoder,
        hardware: &taro_renderer::TaroHardware,
    ) {
        let mut unlit_renderers = pearls.collect_mut::<TaroMeshRenderer<UnlitShader>>();
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

        for renderer in unlit_renderers.iter_mut() {
            renderer.render(&mut pass, camera_matrix, hardware);
        }
    }
}
