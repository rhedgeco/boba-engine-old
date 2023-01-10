use taro_renderer::{
    pearls::TaroMeshRenderer,
    shading::{buffers::CameraMatrix, data_types::DepthView, Taro},
    wgpu, TaroRenderPass,
};

use crate::UnlitShader;

pub struct UnlitRenderPass;

impl TaroRenderPass for UnlitRenderPass {
    fn render(
        &mut self,
        pearls: &taro_renderer::TaroRenderPearls,
        camera_matrix: &CameraMatrix,
        view: &taro_renderer::wgpu::TextureView,
        depth: &Taro<DepthView>,
        encoder: &mut taro_renderer::wgpu::CommandEncoder,
        hardware: &taro_renderer::TaroHardware,
    ) {
        let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth.get_or_compile(hardware),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        });

        let mut unlit_renderers = pearls.collect_mut::<TaroMeshRenderer<UnlitShader>>();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Unlit Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment,
        });

        for renderer in unlit_renderers.iter_mut() {
            renderer.render(&mut pass, camera_matrix, hardware);
        }
    }
}
