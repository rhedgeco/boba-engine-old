use std::sync::Arc;

use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;

use crate::{
    data::{buffers::TransformMatrix, Buffer, Mesh, UniformBinding},
    Bind, Taro, TaroHardware,
};

pub struct TaroMeshRenderer<Shader> {
    model_matrix: Taro<UniformBinding<TransformMatrix>>,

    pub transform: Pearl<BobaTransform>,
    pub shader: Arc<Shader>,
    pub mesh: Taro<Mesh>,
}

impl<Shader> TaroMeshRenderer<Shader> {
    pub fn new(transform: Pearl<BobaTransform>, mesh: Taro<Mesh>, shader: Arc<Shader>) -> Self {
        Self {
            mesh,
            transform,
            shader,
            model_matrix: Bind::new(Buffer::new(wgpu::BufferUsages::empty())),
        }
    }

    pub fn new_simple(transform: BobaTransform, mesh: Taro<Mesh>, shader: Arc<Shader>) -> Self {
        Self::new(Pearl::wrap(transform), mesh, shader)
    }

    pub fn get_updated_model_matrix(
        &self,
        hardware: &TaroHardware,
    ) -> &Taro<UniformBinding<TransformMatrix>> {
        match self.transform.borrow() {
            Ok(t) => {
                let matrix: TransformMatrix = t.world_matrix().into();
                self.model_matrix
                    .get_bind_data()
                    .write_to_hardware(matrix.into(), hardware);
            }
            Err(e) => {
                error!(
                    "Error when recalculating model matrix. Old matrix will be used. Error: {e}"
                );
            }
        };

        &self.model_matrix
    }
}
