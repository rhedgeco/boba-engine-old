use milk_tea::boba_core::{macros::Pearl, pearls::Pearl};
use wgpu::RenderPass;

#[derive(Pearl, Default)]
pub struct TaroCamera {
    target: Option<String>,
}

impl TaroCamera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_target(target: &str) -> Self {
        Self {
            target: Some(target.into()),
        }
    }

    pub fn target(&self) -> Option<&str> {
        let target = self.target.as_ref()?;
        Some(target.as_str())
    }

    pub fn has_target(&self, target: &str) -> bool {
        let Some(this_target) = self.target() else { return false };
        this_target == target
    }

    pub fn set_target(&mut self, target: &str) {
        self.target = Some(target.into());
    }

    pub fn render(&mut self, render_pass: &mut RenderPass) {
        let _ = render_pass;
    }
}
