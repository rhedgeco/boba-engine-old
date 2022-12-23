use boba_3d::glam::{Mat4, Quat, Vec3};
use boba_3d::pearls::BobaTransform;
use boba_core::{BobaEvent, Pearl, PearlRegister, PearlStage};
use log::error;
use milk_tea_runner::events::MilkTeaResize;
use wgpu::{CommandEncoder, TextureView};

use crate::{RenderPearls, RenderPhaseStorage, TaroHardware};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

#[derive(Clone)]
pub struct TaroCameraSettings {
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct TaroCamera {
    transform: Pearl<BobaTransform>,
    matrix: Mat4,

    pub phases: RenderPhaseStorage,
    pub settings: TaroCameraSettings,
}

impl PearlRegister for TaroCamera {
    fn register(pearl: Pearl<Self>, storage: &mut boba_core::storage::StageRunners) {
        storage.add(pearl);
    }
}

impl PearlStage<BobaEvent<MilkTeaResize>> for TaroCamera {
    fn update(
        &mut self,
        data: &MilkTeaResize,
        _: &mut boba_core::BobaResources,
    ) -> boba_core::PearlResult {
        let size = data.size();
        self.settings.aspect = size.width as f32 / size.height as f32;
        Ok(())
    }
}

impl TaroCamera {
    pub fn new(transform: Pearl<BobaTransform>, settings: TaroCameraSettings) -> Self {
        let matrix = match transform.data() {
            Ok(transform) => Self::calculate_matrix(
                transform.world_position(),
                transform.world_rotation(),
                &settings,
            ),
            Err(_) => {
                error!("Error when creating camera. Transform is borrowed as mut. Resorting to default matrix");
                Self::calculate_matrix(Vec3::ZERO, Quat::IDENTITY, &settings)
            }
        };

        Self {
            transform,
            matrix,
            phases: Default::default(),
            settings,
        }
    }

    pub fn execute_render_phases(
        &mut self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
        hardware: &TaroHardware,
    ) {
        if let Ok(transform) = self.transform.data() {
            self.matrix = Self::calculate_matrix(
                transform.world_position(),
                transform.world_rotation(),
                &self.settings,
            )
        } else {
            error!("Error when recalculating camera matrix. Transform is borrowed as mut.");
        }

        self.phases
            .execute_phases(view, &self.matrix, encoder, pearls, hardware);
    }

    pub fn calculate_matrix(position: Vec3, rotation: Quat, settings: &TaroCameraSettings) -> Mat4 {
        let target = position + rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(
            settings.fovy,
            settings.aspect,
            settings.znear,
            settings.zfar,
        );

        proj * view
    }
}
