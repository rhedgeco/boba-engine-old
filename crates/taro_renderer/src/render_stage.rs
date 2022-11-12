use wgpu::CommandEncoder;

use crate::RenderControllers;

pub trait TaroRenderStage {
    fn render(&mut self, encoder: CommandEncoder, controllers: &mut RenderControllers);
}
