use boba_3d::{
    glam::{Mat4, Quat, Vec3},
    pearls::BobaTransform,
};
use boba_core::Pearl;
use log::error;

use crate::{
    data_types::TaroMesh,
    shading::{TaroMeshShader, TaroShader},
    TaroHardware,
};

pub struct TaroMeshRenderer<T> {
    model_matrix: Mat4,

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
            Ok(transform) => transform.world_matrix(),
            Err(e) => {
                error!("Error when creating mesh renderer. Resorting to default model matrix. Error: {e}");
                Mat4::from_scale_rotation_translation(Vec3::ONE, Quat::IDENTITY, Vec3::ZERO)
            }
        };

        Self {
            model_matrix,
            mesh,
            shader,
            transform,
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        pass: &mut wgpu::RenderPass<'pass>,
        camera_matrix: &Mat4,
        hardware: &TaroHardware,
    ) {
        match self.transform.borrow() {
            Ok(t) => self.model_matrix = t.world_matrix(),
            Err(e) => {
                error!("Error when recalculating model matrix. Error: {e}")
            }
        }

        self.shader.upload(hardware).render(
            pass,
            &self.mesh,
            camera_matrix,
            &self.model_matrix,
            hardware,
        );
    }
}
