use wgpu::{CommandBuffer, CommandEncoder, Device, Queue, TextureView};

pub struct TaroRender {
    target: String,
    size: (u32, u32),
    view: TextureView,
    device: Device,
    queue: Queue,
    encoders: Vec<CommandBuffer>,
    redraw: bool,
}

impl TaroRender {
    pub(crate) fn new(
        target: String,
        size: (u32, u32),
        view: TextureView,
        device: Device,
        queue: Queue,
    ) -> Self {
        Self {
            target,
            size,
            view,
            device,
            queue,
            encoders: Vec::new(),
            redraw: false,
        }
    }

    pub(crate) fn submit(self) -> (Device, Queue) {
        self.queue.submit(self.encoders.into_iter());
        (self.device, self.queue)
    }

    pub(crate) fn should_redraw_immediate(&self) -> bool {
        self.redraw
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn image_size(&self) -> (u32, u32) {
        self.size
    }

    pub fn image_width(&self) -> u32 {
        self.size.0
    }

    pub fn image_height(&self) -> u32 {
        self.size.1
    }

    pub fn output_view(&self) -> &TextureView {
        &self.view
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn queue_encoder(&mut self, encoder: CommandEncoder) {
        self.encoders.push(encoder.finish());
    }

    pub fn set_redraw_immediate(&mut self) {
        self.redraw = true;
    }
}
