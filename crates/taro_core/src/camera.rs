use boba_3d::{
    glam::{Mat4, Quat, Vec3},
    pearls::BobaTransform,
};
use boba_core::Pearl;

use crate::{
    buffers::{CameraMatrix, Uniform},
    Buffer, Taro,
};

#[derive(Debug, Clone)]
pub struct TaroCameraSettings {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct TaroCamera {
    aspect_ratio: f32,
    camera_matrix: Taro<Buffer<Uniform<CameraMatrix>>>,

    pub transform: Pearl<BobaTransform>,
    pub settings: TaroCameraSettings,
}

impl TaroCamera {
    pub fn new(transform: Pearl<BobaTransform>, settings: TaroCameraSettings) -> Self {
        let camera_matrix = Buffer::new("Taro Camera Matrix".into(), wgpu::BufferUsages::UNIFORM);

        Self {
            aspect_ratio: 1.,
            camera_matrix,
            transform,
            settings,
        }
    }

    pub fn new_simple(transform: BobaTransform) -> Self {
        Self::new(
            Pearl::wrap(transform),
            TaroCameraSettings {
                fovy: 60.0,
                znear: 0.1,
                zfar: 100.0,
            },
        )
    }

    pub fn calculate_matrix(
        position: Vec3,
        rotation: Quat,
        aspect: f32,
        settings: &TaroCameraSettings,
    ) -> CameraMatrix {
        let target = position + rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(
            settings.fovy.to_radians(),
            aspect,
            settings.znear,
            settings.zfar,
        );

        (proj * view).into()
    }
}
