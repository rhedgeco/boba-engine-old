use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;

use crate::{
    data_types::TaroMesh,
    shading::{
        buffers::{CameraMatrix, TransformMatrix},
        TaroCoreShader, TaroDataUploader, TaroMap, TaroMeshShader, TaroShader,
    },
    TaroHardware,
};

pub struct TaroMeshRenderer<T>
where
    T: TaroCoreShader,
{
    map: TaroMap<TaroMesh>,
    model_matrix: TransformMatrix,

    pub mesh: TaroMesh,
    pub shader: TaroShader<T>,
    pub transform: Pearl<BobaTransform>,
}

impl<T> TaroMeshRenderer<T>
where
    T: TaroMeshShader,
{
    pub fn new(transform: Pearl<BobaTransform>, mesh: TaroMesh, shader: TaroShader<T>) -> Self {
        Self {
            map: Default::default(),
            model_matrix: TransformMatrix::default(),
            mesh,
            shader,
            transform,
        }
    }

    pub fn new_simple(transform: BobaTransform, mesh: TaroMesh, shader: TaroShader<T>) -> Self {
        Self {
            map: Default::default(),
            model_matrix: TransformMatrix::default(),
            mesh,
            shader,
            transform: Pearl::wrap(transform),
        }
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

        let uploaded_mesh = self
            .map
            .get_or_upload(|| self.mesh.new_upload(hardware), hardware);

        let shader = self.shader.get(hardware);
        shader.set_camera_matrix(camera_matrix, hardware);
        shader.set_model_matrix(&self.model_matrix, hardware);
        shader.render(pass, uploaded_mesh, hardware);
    }
}
