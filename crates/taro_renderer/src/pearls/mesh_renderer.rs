use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;

use crate::{
    data_types::{
        buffers::{CameraMatrix, TransformMatrix},
        TaroMesh,
    },
    shading::{TaroBuffer, TaroCoreShader, TaroDataUploader, TaroMap, TaroMeshShader, TaroShader},
    TaroHardware,
};

pub struct TaroMeshRenderer<T>
where
    T: TaroCoreShader,
{
    map: TaroMap<TaroMesh>,
    model_matrix: TaroMap<TransformMatrix>,

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
            model_matrix: TaroMap::new(),
            mesh,
            shader,
            transform,
        }
    }

    pub fn new_simple(transform: BobaTransform, mesh: TaroMesh, shader: TaroShader<T>) -> Self {
        Self {
            map: Default::default(),
            model_matrix: TaroMap::new(),
            mesh,
            shader,
            transform: Pearl::wrap(transform),
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        pass: &mut wgpu::RenderPass<'pass>,
        camera_matrix: &'pass TaroBuffer<CameraMatrix>,
        hardware: &TaroHardware,
    ) {
        let model_matrix = match self.transform.borrow() {
            Ok(t) => self
                .model_matrix
                .upload_new(&t.world_matrix().into(), hardware),
            Err(e) => {
                error!("Error when recalculating model matrix. Error: {e}");
                self.model_matrix
                    .get_or_upload(|| TransformMatrix::default().new_upload(hardware), hardware)
            }
        };

        let uploaded_mesh = self
            .map
            .get_or_upload(|| self.mesh.new_upload(hardware), hardware);
        self.shader.upload(hardware).render(
            pass,
            uploaded_mesh,
            camera_matrix,
            model_matrix,
            hardware,
        );
    }
}
