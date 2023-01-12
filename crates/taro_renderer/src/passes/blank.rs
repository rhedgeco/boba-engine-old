use crate::{
    shading::{buffers::CameraMatrix, DepthView, Taro},
    TaroRenderPass,
};

pub struct BlankRenderPass;

impl TaroRenderPass for BlankRenderPass {
    fn render(
        &mut self,
        _pearls: &crate::TaroRenderPearls,
        _camera_matrix: &CameraMatrix,
        view: &wgpu::TextureView,
        _depth: &Taro<DepthView>,
        encoder: &mut wgpu::CommandEncoder,
        _hardware: &crate::TaroHardware,
    ) {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Blank Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }
}
