use boba_3d::{
    glam::{Mat4, Quat, Vec3},
    pearls::BobaTransform,
};
use boba_core::Pearl;
use log::error;
use wgpu::RenderPass;

use crate::{
    shading::{TaroMeshShader, TaroShader},
    types::TaroMesh,
    TaroHardware,
};

pub struct TaroMeshRenderer<T>
where
    T: TaroMeshShader,
{
    pub transform: Pearl<BobaTransform>,
    matrix: Mat4,

    pub mesh: TaroMesh,
    pub shader: TaroShader<T>,
}

impl<T> TaroMeshRenderer<T>
where
    T: TaroMeshShader,
{
    pub fn new(transform: Pearl<BobaTransform>, mesh: TaroMesh, shader: TaroShader<T>) -> Self {
        let matrix = match transform.data() {
            Ok(transform) => transform.world_matrix(),
            Err(_) => {
                error!("Error when creating mesh renderer. Transform is borrowed as mut. Resorting to default matrix");
                Mat4::from_scale_rotation_translation(Vec3::ONE, Quat::IDENTITY, Vec3::ZERO)
            }
        };

        Self {
            transform,
            matrix,
            mesh,
            shader,
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        pass: &mut RenderPass<'pass>,
        camera_matrix: &Mat4,
        hardware: &TaroHardware,
    ) {
        if let Ok(transform) = self.transform.data() {
            self.matrix = transform.world_matrix()
        } else {
            error!("Error when recalculating camera matrix. Transform is borrowed as mut.");
        }

        let shader = self.shader.compile(hardware);
        let mesh = self.mesh.compile(hardware);
        shader.render(pass, mesh, camera_matrix, &self.matrix, hardware);
    }
}
