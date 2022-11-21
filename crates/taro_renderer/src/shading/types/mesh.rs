use wgpu::{BindGroup, RenderPass};

use crate::{shading::TaroShaderCore, types::CompiledTaroMesh};

pub struct RenderMeshData<'a> {
    pub mesh: &'a CompiledTaroMesh,
    pub camera_bind_group: &'a BindGroup,
    pub model_bind_group: &'a BindGroup,
}

pub trait TaroMeshShader: TaroShaderCore {
    fn render_mesh<'a>(&'a self, pass: &mut RenderPass<'a>, data: &RenderMeshData<'a>);
}
