use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;

use crate::{
    data_types::TaroMesh,
    shading::{
        bindings::{CameraMatrix, TransformMatrix},
        TaroMap, TaroMeshShader, TaroShader,
    },
    TaroHardware,
};

pub struct TaroMeshRenderer<T> {
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
        let model_matrix = match transform.borrow() {
            Ok(transform) => transform.world_matrix().into(),
            Err(e) => {
                error!("Error when creating mesh renderer. Resorting to default model matrix. Error: {e}");
                TransformMatrix::default()
            }
        };

        Self {
            map: Default::default(),
            model_matrix,
            mesh,
            shader,
            transform,
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
                error!("Error when recalculating model matrix. Error: {e}")
            }
        }

        let uploaded_mesh = self.map.get_or_upload(&self.mesh, hardware);
        self.shader.upload(hardware).render(
            pass,
            uploaded_mesh,
            camera_matrix,
            &self.model_matrix,
            hardware,
        );
    }
}
