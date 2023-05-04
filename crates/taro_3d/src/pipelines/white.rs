use boba_3d::glam::Mat4;
use boba_core::BobaEventData;
use taro_renderer::{events::TaroRender, wgpu};

use crate::TaroPipeline;

pub struct WhitePipeline;

impl TaroPipeline for WhitePipeline {
    fn render(&mut self, _: &Mat4, data: &mut BobaEventData<TaroRender>) {
        let device = data.event.hardware().device();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("White Stage Encoder"),
        });

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("White Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: data.event.output_view(),
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

        data.event.queue_encoder(encoder);
    }
}
