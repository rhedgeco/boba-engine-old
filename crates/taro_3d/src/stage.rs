use boba_3d::glam::Mat4;
use boba_core::{pearls::map::EventPearls, BobaResources};

pub trait RenderStage: 'static {
    fn render(
        &mut self,
        view_proj_mat: &Mat4,
        pearls: &mut EventPearls,
        resources: &mut BobaResources,
    );
}

pub struct RenderStages {
    stages: Vec<Box<dyn RenderStage>>,
}

impl Default for RenderStages {
    fn default() -> Self {
        Self { stages: Vec::new() }
    }
}

impl RenderStages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, stage: impl RenderStage) {
        self.stages.push(Box::new(stage));
    }

    pub fn render_all(
        &mut self,
        view_proj_mat: &Mat4,
        pearls: &mut EventPearls,
        resources: &mut BobaResources,
    ) {
        for stage in self.stages.iter_mut() {
            stage.render(view_proj_mat, pearls, resources);
        }
    }
}
