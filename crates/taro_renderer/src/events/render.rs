use wgpu::{CommandEncoder, Queue, RenderPass, RenderPassDescriptor};

pub struct TaroRender {
    target: String,
    size: (u32, u32),
    encoder: CommandEncoder,
    redraw: bool,
}

impl TaroRender {
    pub(crate) fn new(target: String, size: (u32, u32), encoder: CommandEncoder) -> Self {
        Self {
            target,
            size,
            encoder,
            redraw: false,
        }
    }

    pub(crate) fn submit(self, queue: &Queue) {
        queue.submit(std::iter::once(self.encoder.finish()));
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

    pub fn set_immediate_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn begin_render_pass<'pass>(
        &'pass mut self,
        desc: &RenderPassDescriptor<'pass, '_>,
    ) -> RenderPass<'pass> {
        self.encoder.begin_render_pass(desc)
    }
}
