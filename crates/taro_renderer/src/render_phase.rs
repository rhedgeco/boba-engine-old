use std::any::TypeId;

use indexmap::IndexMap;
use wgpu::{BindGroup, CommandEncoder, TextureView};

use crate::{RenderPearls, RenderResources};

pub trait TaroRenderPhase {
    fn render(
        &mut self,
        view: &TextureView,
        camera: &BindGroup,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
        resources: &RenderResources,
    );
}

pub struct RenderPhaseStorage {
    phases: IndexMap<TypeId, Box<dyn TaroRenderPhase>>,
}

impl Default for RenderPhaseStorage {
    fn default() -> Self {
        Self {
            phases: Default::default(),
        }
    }
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
        camera: &BindGroup,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
        resources: &RenderResources,
    ) {
        for phase in self.phases.values_mut() {
            phase.render(view, camera, encoder, pearls, resources);
        }
    }
}
