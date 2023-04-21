use wgpu::{CommandEncoder, Queue, RenderPass, RenderPassDescriptor};

pub struct TaroRender {
    target: String,
    encoder: CommandEncoder,
    redraw: bool,
}

impl TaroRender {
    pub(crate) fn new(target: String, encoder: CommandEncoder) -> Self {
        Self {
            target,
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
