use boba_3d::{
    glam::{Mat4, Quat, Vec3},
    pearls::BobaTransform,
};
use boba_core::Pearl;
use log::error;

use crate::{TaroHardware, TaroRenderPasses, TaroRenderPearls};

#[derive(Default)]
pub struct TaroCameras {
    pub cameras: Vec<TaroCamera>,
}

#[derive(Debug, Clone)]
pub struct TaroCameraSettings {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct TaroCamera {
    pub(crate) aspect: f32,
    view_proj_matrix: Mat4,

    pub transform: Pearl<BobaTransform>,
    pub settings: TaroCameraSettings,
    pub passes: TaroRenderPasses,
}

impl TaroCamera {
    pub fn new(settings: TaroCameraSettings, transform: Pearl<BobaTransform>) -> Self {
        let aspect = 1f32;
        let view_proj_matrix = match transform.borrow() {
            Ok(transform) => Self::calculate_matrix(
                transform.world_position(),
                transform.world_rotation(),
                aspect,
                &settings,
            ),
            Err(e) => {
                error!("Error when creating camera. Resorting to default view-projection matrix. Error: {e}");
                Self::calculate_matrix(Vec3::ZERO, Quat::IDENTITY, aspect, &settings)
            }
        };

        Self {
            aspect,
            view_proj_matrix,
            transform,
            settings,
            passes: Default::default(),
        }
    }

    pub fn render(
        &mut self,
        pearls: &TaroRenderPearls,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        hardware: &TaroHardware,
    ) {
        match self.transform.borrow() {
            Ok(t) => {
                self.view_proj_matrix = Self::calculate_matrix(
                    t.world_position(),
                    t.world_rotation(),
                    self.aspect,
                    &self.settings,
                );
            }
            Err(e) => {
                error!(
                    "Error when recalculating camera matrix. Resorting to previously calculated matrix. Error: {e}"
                )
            }
        }

        self.passes
            .render(pearls, &self.view_proj_matrix, view, encoder, hardware);
    }

    pub fn calculate_matrix(
        position: Vec3,
        rotation: Quat,
        aspect: f32,
        settings: &TaroCameraSettings,
    ) -> Mat4 {
        let target = position + rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(settings.fovy, aspect, settings.znear, settings.zfar);

        view * proj
    }
}
