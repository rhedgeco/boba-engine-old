use boba_3d::glam::Mat4;

use crate::data::BytesBuilder;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TransformMatrix {
    matrix_data: [[f32; 4]; 4],
}

impl Default for TransformMatrix {
    fn default() -> Self {
        Self {
            matrix_data: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

impl BytesBuilder for TransformMatrix {
    const LABEL: &'static str = "Transform";
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

impl From<&Mat4> for TransformMatrix {
    fn from(value: &Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}
