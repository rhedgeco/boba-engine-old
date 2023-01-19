use boba_3d::glam::{Mat4, Vec3};

use crate::data::BytesBuilder;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TransformMatrix {
    matrix_data: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraMatrix {
    matrix_data: [[f32; 4]; 4],
}

impl Default for TransformMatrix {
    fn default() -> Self {
        Self {
            matrix_data: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

impl Default for CameraMatrix {
    fn default() -> Self {
        Self {
            matrix_data: Mat4::look_at_rh(Vec3::ZERO, Vec3::Z, Vec3::Y).to_cols_array_2d(),
        }
    }
}

impl BytesBuilder for TransformMatrix {
    const LABEL: &'static str = "TransformMatrix";

    fn build_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.matrix_data)
    }
}

impl BytesBuilder for CameraMatrix {
    const LABEL: &'static str = "CameraMatrix";

    fn build_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.matrix_data)
    }
}

impl From<Mat4> for TransformMatrix {
    fn from(value: Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}

impl From<Mat4> for CameraMatrix {
    fn from(value: Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}

impl From<&Mat4> for TransformMatrix {
    fn from(value: &Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}

impl From<&Mat4> for CameraMatrix {
    fn from(value: &Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}
