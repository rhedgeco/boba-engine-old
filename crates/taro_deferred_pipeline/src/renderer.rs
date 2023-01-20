use std::{ops::Deref, sync::Arc};

use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;
use taro_core::{
    data::{buffers::TransformMatrix, Buffer, Mesh, Uniform},
    wgpu, Bind, Taro, TaroHardware,
};

use crate::shaders::DeferredShader;

pub struct DeferredRenderer {
    pub transform: Pearl<BobaTransform>,
    pub mesh: Taro<Mesh>,

    shader: Arc<dyn DeferredShader>,
    model_matrix: Taro<Bind<Buffer<Uniform<TransformMatrix>>>>,
}

impl Deref for DeferredRenderer {
    type Target = Arc<dyn DeferredShader>;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}

impl DeferredRenderer {
    pub fn new(
        transform: Pearl<BobaTransform>,
        mesh: Taro<Mesh>,
        shader: Arc<impl DeferredShader>,
    ) -> Self {
        Self {
            mesh,
            transform,
            shader,
            model_matrix: Bind::new(Buffer::new(wgpu::BufferUsages::empty())),
        }
    }

    pub fn new_simple(
        transform: BobaTransform,
        mesh: Taro<Mesh>,
        shader: Arc<impl DeferredShader>,
    ) -> Self {
        Self::new(Pearl::wrap(transform), mesh, shader)
    }

    pub fn get_updated_model_matrix(
        &self,
        hardware: &TaroHardware,
    ) -> &Taro<Bind<Buffer<Uniform<TransformMatrix>>>> {
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
