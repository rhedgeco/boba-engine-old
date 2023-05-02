use boba_3d::glam::Mat4;
use boba_core::BobaEventData;
use taro_renderer::{events::TaroRender, wgpu};

use crate::{TaroPipeline, TaroSkybox};

pub struct UnlitPipeline;

impl TaroPipeline for UnlitPipeline {
    fn render(&mut self, _: &Mat4, event: &mut BobaEventData<TaroRender>) {
        let device = event.hardware().device();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("White Stage Encoder"),
        });

        event.resources.get_mut_and::<TaroSkybox>(|skybox| {
            let color = skybox.wgpu_color();

            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("White Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: event.event.output_view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        });

        event.queue_encoder(encoder);
    }
}
