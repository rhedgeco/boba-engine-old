use std::any::TypeId;

use boba_3d::glam::Mat4;
use indexmap::IndexMap;
use wgpu::{CommandEncoder, TextureView};

use crate::{RenderPearls, TaroHardware};

pub trait TaroRenderPhase {
    fn render(
        &mut self,
        view: &TextureView,
        camera_matrix: &Mat4,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
        hardware: &TaroHardware,
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
        camera_matrix: &Mat4,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
        hardware: &TaroHardware,
    ) {
        for phase in self.phases.values_mut() {
            phase.render(view, camera_matrix, encoder, pearls, hardware);
        }
    }
}
