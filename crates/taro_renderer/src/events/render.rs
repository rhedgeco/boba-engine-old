use milk_tea::boba_core::Event;
use wgpu::{CommandBuffer, CommandEncoder, SurfaceTexture, TextureView};

use crate::TaroHardware;

pub struct TaroRender {
    name: String,
    surface: SurfaceTexture,
    view: TextureView,
    buffers: Vec<CommandBuffer>,
}

impl TaroRender {
    pub(crate) fn new(name: String, surface: SurfaceTexture) -> Self {
        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            name,
            surface,
            view,
            buffers: Vec::new(),
        }
    }

    pub fn event_data<'a>(&'a mut self, hardware: &'a TaroHardware) -> TaroRenderData {
        TaroRenderData {
            event: self,
            hardware,
        }
    }

    pub fn submit(mut self, hardware: &TaroHardware) {
        if self.buffers.is_empty() {
            let device = hardware.device();
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Default Empty Encoder"),
            });

            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Empty Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.buffers.push(encoder.finish());
        }

        hardware.queue().submit(self.buffers.into_iter());
        self.surface.present();
    }
}

impl Event for TaroRender {
    type Data<'a> = TaroRenderData<'a>;
}

pub struct TaroRenderData<'a> {
    event: &'a mut TaroRender,
    hardware: &'a TaroHardware,
}

impl<'a> TaroRenderData<'a> {
    pub fn window_name(&self) -> &str {
        &self.event.name
    }

    pub fn image_width(&self) -> u32 {
        self.event.surface.texture.width()
    }

    pub fn image_height(&self) -> u32 {
        self.event.surface.texture.height()
    }

    pub fn output_view(&self) -> &TextureView {
        &self.event.view
    }

    pub fn hardware(&self) -> &TaroHardware {
        &self.hardware
    }

    pub fn queue_encoder(&mut self, encoder: CommandEncoder) {
        self.event.buffers.push(encoder.finish());
    }
}
