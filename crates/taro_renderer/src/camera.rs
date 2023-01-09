use boba_3d::{
    glam::{Mat4, Quat, Vec3},
    pearls::BobaTransform,
};
use boba_core::Pearl;
use log::error;

use crate::{shading::buffers::CameraMatrix, TaroHardware, TaroRenderPasses, TaroRenderPearls};

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
    camera_matrix: CameraMatrix,

    pub(crate) aspect: f32,
    pub transform: Pearl<BobaTransform>,
    pub settings: TaroCameraSettings,
    pub passes: TaroRenderPasses,
}

impl TaroCamera {
    pub fn new(transform: Pearl<BobaTransform>, settings: TaroCameraSettings) -> Self {
        Self {
            aspect: 1.,
            camera_matrix: CameraMatrix::default(),
            transform,
            settings,
            passes: Default::default(),
        }
    }

    pub fn new_simple(transform: BobaTransform, settings: TaroCameraSettings) -> Self {
        Self {
            aspect: 1.,
            camera_matrix: CameraMatrix::default(),
            transform: Pearl::wrap(transform),
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
                self.camera_matrix = Self::calculate_matrix(
                    t.world_position(),
                    t.world_rotation(),
                    self.aspect,
                    &self.settings,
                );
            }
            Err(e) => {
                error!("Error when calculating camera matrix. Error: {e}");
            }
        };

        self.passes
            .render(pearls, &self.camera_matrix, view, encoder, hardware);
    }

    pub fn calculate_matrix(
        position: Vec3,
        rotation: Quat,
        aspect: f32,
        settings: &TaroCameraSettings,
    ) -> CameraMatrix {
        let target = position + rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(settings.fovy, aspect, settings.znear, settings.zfar);

        (proj * view).into()
    }
}
