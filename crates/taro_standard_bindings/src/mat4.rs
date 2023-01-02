use boba_3d::glam::Mat4;
use taro_renderer::shading::TaroBindingBuilder;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Mat4Binding {
    matrix_data: [[f32; 4]; 4],
}

impl TaroBindingBuilder for Mat4Binding {
    fn build_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.matrix_data)
    }
}

impl From<Mat4> for Mat4Binding {
    fn from(value: Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}

impl From<&Mat4> for Mat4Binding {
    fn from(value: &Mat4) -> Self {
        Self {
            matrix_data: value.to_cols_array_2d(),
        }
    }
}
