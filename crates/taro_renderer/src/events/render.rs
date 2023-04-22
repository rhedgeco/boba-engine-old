// use wgpu::{CommandBuffer, CommandEncoder, SurfaceTexture, TextureView};

// use crate::TaroHardware;

// pub struct TaroRender {
//     target: String,
//     output: SurfaceTexture,
//     view: TextureView,
//     hardware: TaroHardware,
//     encoders: Vec<CommandBuffer>,
//     redraw: bool,
// }

// impl TaroRender {
//     pub(crate) fn new(target: String, output: SurfaceTexture, hardware: TaroHardware) -> Self {
//         let view = output
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());

//         Self {
//             target,
//             output,
//             view,
//             hardware,
//             encoders: Vec::new(),
//             redraw: false,
//         }
//     }

//     pub(crate) fn submit(self) -> TaroHardware {
//         if !self.encoders.is_empty() {
//             self.hardware.queue.submit(self.encoders.into_iter());
//             self.output.present();
//         }

//         self.hardware
//     }

//     pub(crate) fn should_redraw_immediate(&self) -> bool {
//         self.redraw
//     }

//     pub fn target(&self) -> &str {
//         &self.target
//     }

//     pub fn image_size(&self) -> (u32, u32) {
//         (self.image_width(), self.image_height())
//     }

//     pub fn image_width(&self) -> u32 {
//         self.output.texture.width()
//     }

//     pub fn image_height(&self) -> u32 {
//         self.output.texture.height()
//     }

//     pub fn output_view(&self) -> &TextureView {
//         &self.view
//     }

//     pub fn hardware(&self) -> &TaroHardware {
//         &self.hardware
//     }

//     pub fn queue_encoder(&mut self, encoder: CommandEncoder) {
//         self.encoders.push(encoder.finish());
//     }

//     pub fn set_redraw_immediate(&mut self) {
//         self.redraw = true;
//     }
// }

use milk_tea::winit::window::WindowId;
use wgpu::{CommandBuffer, CommandEncoder, SurfaceTexture, TextureView};

use crate::TaroHardware;

pub struct TaroRender {
    name: String,
    window_id: WindowId,
    surface: SurfaceTexture,
    view: TextureView,
    hardware: TaroHardware,
    buffers: Vec<CommandBuffer>,
    redraw: bool,
}

impl TaroRender {
    pub(crate) fn new(
        name: String,
        window_id: WindowId,
        surface: SurfaceTexture,
        hardware: TaroHardware,
    ) -> Self {
        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            name,
            window_id,
            surface,
            view,
            hardware,
            buffers: Vec::new(),
            redraw: false,
        }
    }

    pub(crate) fn submit(self) -> TaroHardware {
        if !self.buffers.is_empty() {
            self.hardware.queue().submit(self.buffers.into_iter());
            self.surface.present();
        }

        self.hardware
    }

    pub(crate) fn immediate_redraw_requested(&self) -> bool {
        self.redraw
    }

    pub fn window_name(&self) -> &str {
        &self.name
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub fn image_width(&self) -> u32 {
        self.surface.texture.width()
    }

    pub fn image_height(&self) -> u32 {
        self.surface.texture.height()
    }

    pub fn output_view(&self) -> &TextureView {
        &self.view
    }

    pub fn hardware(&self) -> &TaroHardware {
        &self.hardware
    }

    pub fn queue_encoder(&mut self, encoder: CommandEncoder) {
        self.buffers.push(encoder.finish());
    }

    pub fn request_immediate_redraw(&mut self) {
        self.redraw = true;
    }
}
