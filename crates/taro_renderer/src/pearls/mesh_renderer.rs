use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;

use crate::{
    shading::{
        buffers::{CameraMatrix, TransformMatrix},
        data_types::Mesh,
        Taro, TaroCoreShader, TaroMeshShader, TaroShader,
    },
    TaroHardware,
};

pub struct TaroMeshRenderer<T>
where
    T: TaroCoreShader,
{
    model_matrix: TransformMatrix,

    pub mesh: Taro<Mesh>,
    pub shader: TaroShader<T>,
    pub transform: Pearl<BobaTransform>,
}

impl<T> TaroMeshRenderer<T>
where
    T: TaroMeshShader,
{
    pub fn new(transform: Pearl<BobaTransform>, mesh: Taro<Mesh>, shader: TaroShader<T>) -> Self {
        Self {
            model_matrix: TransformMatrix::default(),
            mesh,
            shader,
            transform,
        }
    }

    pub fn new_simple(transform: BobaTransform, mesh: Taro<Mesh>, shader: TaroShader<T>) -> Self {
        Self::new(Pearl::wrap(transform), mesh, shader)
    }

    pub fn render<'pass>(
        &'pass mut self,
        pass: &mut wgpu::RenderPass<'pass>,
        camera_matrix: &CameraMatrix,
        hardware: &TaroHardware,
    ) {
        match self.transform.borrow() {
            Ok(t) => self.model_matrix = t.world_matrix().into(),
            Err(e) => {
                error!("Error when recalculating model matrix. Error: {e}");
            }
        };

        let uploaded_mesh = self.mesh.get_or_compile(hardware);
        let shader = self.shader.get(hardware);
        shader.set_camera_matrix(camera_matrix, hardware);
        shader.set_model_matrix(&self.model_matrix, hardware);
        shader.render(pass, uploaded_mesh, hardware);
    }
}
