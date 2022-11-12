use std::any::TypeId;

use indexmap::IndexMap;
use wgpu::{CommandEncoder, TextureView};

use crate::RenderControllers;

pub trait TaroRenderPhase {
    fn render(
        &mut self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
        controllers: &mut RenderControllers,
    );
}

#[derive(Default)]
pub struct RenderPhaseStorage {
    phases: IndexMap<TypeId, Box<dyn TaroRenderPhase>>,
}

impl RenderPhaseStorage {
    pub fn add<T: 'static + TaroRenderPhase>(&mut self, phase: T) {
        self.phases.insert(TypeId::of::<T>(), Box::new(phase));
    }

    pub fn remove<T: 'static + TaroRenderPhase>(&mut self) {
        self.phases.remove(&TypeId::of::<T>());
    }

    pub fn execute_phases(
        &mut self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
        controllers: &mut RenderControllers,
    ) {
        for phase in self.phases.values_mut() {
            phase.render(view, encoder, controllers);
        }
    }
}
