use wgpu::CommandEncoder;

use crate::RenderControllers;

pub trait TaroRenderPhase {
    fn render(&mut self, encoder: CommandEncoder, controllers: &mut RenderControllers);
}
