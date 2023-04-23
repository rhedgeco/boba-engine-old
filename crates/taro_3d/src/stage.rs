use boba_3d::glam::Mat4;
use boba_core::pearls::map::EventData;
use taro_renderer::events::TaroRender;

use crate::stages::WhiteRenderStage;

pub trait RenderStage: 'static {
    fn render(&mut self, view_proj_mat: &Mat4, event: &mut EventData<TaroRender>);
}

pub struct RenderStages {
    stages: Vec<Box<dyn RenderStage>>,
}

impl Default for RenderStages {
    fn default() -> Self {
        let mut new = Self { stages: Vec::new() };
        new.push(WhiteRenderStage);
        new
    }
}

impl RenderStages {
    pub fn empty() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn push(&mut self, stage: impl RenderStage) {
        self.stages.push(Box::new(stage));
    }

    pub fn render_all(&mut self, view_proj_mat: &Mat4, event: &mut EventData<TaroRender>) {
        for stage in self.stages.iter_mut() {
            stage.render(view_proj_mat, event);
        }
    }
}
