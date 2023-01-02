use boba_3d::glam::Mat4;
use sync_cache::SyncCache;

use crate::{data_types::TaroMesh, HardwareId, TaroHardware};

pub trait TaroCoreShader: 'static {
    fn build_instance(hardware: &TaroHardware) -> Self;
}

pub trait TaroMeshShader: TaroCoreShader {
    fn render<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        mesh: &'pass TaroMesh,
        camera_matrix: &Mat4,
        model_matrix: &Mat4,
        hardware: &TaroHardware,
    );
}

#[derive(Clone)]
pub struct TaroShader<T> {
    shader_cache: SyncCache<HardwareId, T>,
}

impl<T> Default for TaroShader<T> {
    fn default() -> Self {
        Self {
            shader_cache: Default::default(),
        }
    }
}

impl<T> TaroShader<T>
where
    T: TaroCoreShader,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn upload(&self, hardware: &TaroHardware) -> &T {
        self.shader_cache
            .get_or_init(hardware.id(), || T::build_instance(hardware))
    }
}
