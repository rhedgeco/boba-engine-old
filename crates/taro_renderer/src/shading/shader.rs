use once_map::OnceMap;

use crate::{
    data_types::{
        buffers::{CameraMatrix, TransformMatrix},
        TaroBuffer, UploadedTaroMesh,
    },
    HardwareId, TaroHardware,
};

pub trait TaroCoreShader: 'static {
    type InitialParameters;
    fn build_instance(parameters: &Self::InitialParameters, hardware: &TaroHardware) -> Self;
}

pub trait TaroMeshShader: TaroCoreShader {
    fn render<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        mesh: &'pass UploadedTaroMesh,
        camera_matrix: &'pass TaroBuffer<CameraMatrix>,
        model_matrix: &'pass TaroBuffer<TransformMatrix>,
        hardware: &TaroHardware,
    );
}

#[derive(Clone)]
pub struct TaroShader<T>
where
    T: TaroCoreShader,
{
    parameters: T::InitialParameters,
    shader_cache: OnceMap<HardwareId, T>,
}

impl<T> TaroShader<T>
where
    T: TaroCoreShader,
{
    pub fn new(parameters: T::InitialParameters) -> Self {
        Self {
            parameters,
            shader_cache: Default::default(),
        }
    }

    pub fn upload(&self, hardware: &TaroHardware) -> &T {
        self.shader_cache
            .get_or_init(hardware.id(), || {
                T::build_instance(&self.parameters, hardware)
            })
            .into_data()
    }
}
