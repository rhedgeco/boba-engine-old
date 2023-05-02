use milk_tea::{boba_core::Event, winit::window::WindowId};
use wgpu::{CommandBuffer, CommandEncoder, SurfaceTexture, TextureView};

use crate::TaroHardware;

pub struct TaroRender {
    name: String,
    window_id: WindowId,
    surface: SurfaceTexture,
    view: TextureView,
    buffers: Vec<CommandBuffer>,
    redraw: bool,
}

impl TaroRender {
    pub(crate) fn new(name: String, window_id: WindowId, surface: SurfaceTexture) -> Self {
        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            name,
            window_id,
            surface,
            view,
            buffers: Vec::new(),
            redraw: false,
        }
    }

    pub fn event_data<'a>(&'a mut self, hardware: &'a TaroHardware) -> TaroRenderData {
        TaroRenderData {
            render_event: self,
            hardware,
        }
    }

    pub fn submit(self, hardware: &TaroHardware) {
        if !self.buffers.is_empty() {
            hardware.queue().submit(self.buffers.into_iter());
            self.surface.present();
        }
    }

    pub fn immediate_redraw_requested(&self) -> bool {
        self.redraw
    }
}

impl Event for TaroRender {
    type Data<'a> = TaroRenderData<'a>;
}

pub struct TaroRenderData<'a> {
    render_event: &'a mut TaroRender,
    hardware: &'a TaroHardware,
}

impl<'a> TaroRenderData<'a> {
    pub fn window_name(&self) -> &str {
        &self.render_event.name
    }

    pub fn window_id(&self) -> WindowId {
        self.render_event.window_id
    }

    pub fn image_width(&self) -> u32 {
        self.render_event.surface.texture.width()
    }

    pub fn image_height(&self) -> u32 {
        self.render_event.surface.texture.height()
    }

    pub fn output_view(&self) -> &TextureView {
        &self.render_event.view
    }

    pub fn hardware(&self) -> &TaroHardware {
        &self.hardware
    }

    pub fn queue_encoder(&mut self, encoder: CommandEncoder) {
        self.render_event.buffers.push(encoder.finish());
    }

    pub fn request_immediate_redraw(&mut self) {
        self.render_event.redraw = true;
    }
}
