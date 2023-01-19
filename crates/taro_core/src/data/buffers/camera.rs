use boba_3d::glam::{Mat4, Quat, Vec3};

use crate::{data::BytesBuilder, TaroCameraSettings};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraMatrix {
    matrix_data: [[f32; 4]; 4],
}

impl Default for CameraMatrix {
    fn default() -> Self {
        Self {
            matrix_data: Mat4::look_at_rh(Vec3::ZERO, Vec3::Z, Vec3::Y).to_cols_array_2d(),
        }
    }
}

impl BytesBuilder for CameraMatrix {
    const LABEL: &'static str = "Color";
    fn build_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.matrix_data)
    }
}

impl CameraMatrix {
    /// Creates a new camera matrix with the provided properties
    pub fn new(position: Vec3, rotation: Quat, aspect: f32, settings: &TaroCameraSettings) -> Self {
        let target = position + rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(
            settings.fovy.to_radians(),
            aspect,
            settings.znear,
            settings.zfar,
        );

        Self {
            matrix_data: (proj * view).to_cols_array_2d(),
        }
    }
}
