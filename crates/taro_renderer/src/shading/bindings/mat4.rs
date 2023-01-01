use boba_3d::glam::Mat4;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4Binding {
    matrix_data: [[f32; 4]; 4],
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
