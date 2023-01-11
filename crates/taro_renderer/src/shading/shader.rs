use super::{
    buffers::{CameraMatrix, TransformMatrix},
    data_types::MeshBuffer,
    Taro, TaroBuilder,
};
use crate::TaroHardware;

/// The base trait for any shader type.
///
/// To be a shader, this *must* be implemented.
pub trait TaroCoreShader {
    type InitParameters;
    fn build_instance(init: &Self::InitParameters, hardware: &TaroHardware) -> Self;
}

/// The base trait for a shader that can render a mesh on screen.
pub trait TaroMeshShader: TaroCoreShader {
    fn set_camera_matrix(&self, data: &CameraMatrix, hardware: &TaroHardware);
    fn set_model_matrix(&self, data: &TransformMatrix, hardware: &TaroHardware);
    fn render<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        mesh: &'pass MeshBuffer,
        hardware: &TaroHardware,
    );
}

pub struct Shader<T: TaroCoreShader> {
    init: T::InitParameters,
}

impl<T: TaroCoreShader> Shader<T> {
    pub fn new(init: T::InitParameters) -> Taro<Self> {
        Taro::new(Self { init })
    }
}

impl<T: TaroCoreShader> TaroBuilder for Shader<T> {
    type Compiled = T;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        T::build_instance(&self.init, hardware)
    }
}
