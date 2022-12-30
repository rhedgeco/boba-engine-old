use crate::TaroRenderPass;

pub struct BlankRenderPass;

impl TaroRenderPass for BlankRenderPass {
    fn render(
        &mut self,
        _pearls: &crate::TaroRenderPearls,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        _hardware: &crate::TaroHardware,
    ) {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("White Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }
}
