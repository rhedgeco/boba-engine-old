use super::buffers::{CameraMatrix, TransformMatrix};
use crate::{data_types::TaroMeshBuffer, HardwareId, TaroHardware};
use once_map::OnceMap;

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
        mesh: &'pass TaroMeshBuffer,
        hardware: &TaroHardware,
    );
}

/// The main struct to hold and manage shaders for TaroRenderers
#[derive(Clone)]
pub struct TaroShader<T>
where
    T: TaroCoreShader,
{
    parameters: T::InitParameters,
    shader_cache: OnceMap<HardwareId, T>,
}

impl<T> TaroShader<T>
where
    T: TaroCoreShader,
{
    /// Creates a new TaroShader with `init` parameters
    pub fn new(init: T::InitParameters) -> Self {
        Self {
            parameters: init,
            shader_cache: Default::default(),
        }
    }

    /// Gets the compiled shader associated with `hardware`
    ///
    /// If the shader has not been compiled yet, it will be now.
    pub fn get(&self, hardware: &TaroHardware) -> &T {
        self.shader_cache
            .get_or_init(hardware.id().clone(), || {
                T::build_instance(&self.parameters, hardware)
            })
            .into_data()
    }
}
