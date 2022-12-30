use crate::TaroRenderPass;

pub struct WhiteRenderPass;

impl TaroRenderPass for WhiteRenderPass {
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
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }
}
