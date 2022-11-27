use std::cell::BorrowError;

use crate::{
    shading::ShaderId,
    types::{CompiledTaroMesh, TaroMesh},
    RenderHardware,
};
use boba_3d::pearls::BobaTransform;
use boba_core::{Pearl, PearlRegister};
use wgpu::{util::DeviceExt, BindGroup, Buffer};

pub struct TaroMeshBinding<'a> {
    pub mesh: &'a CompiledTaroMesh,
    pub matrix: &'a BindGroup,
}

struct CompiledMatrixData {
    buffer: Buffer,
    binding: BindGroup,
}

pub struct TaroMeshRenderer {
    transform: Pearl<BobaTransform>,
    matrix: Option<CompiledMatrixData>,

    pub shader: ShaderId,
    pub mesh: TaroMesh,
}

impl PearlRegister for TaroMeshRenderer {
    fn register(_: boba_core::Pearl<Self>, _: &mut boba_core::storage::StageRunners) {
        // do nothing
    }
}

impl TaroMeshRenderer {
    pub fn new(transform: Pearl<BobaTransform>, mesh: TaroMesh, shader: ShaderId) -> Self {
        Self {
            transform,
            matrix: None,
            mesh,
            shader,
        }
    }

    pub fn mesh_binding(
        &mut self,
        hardware: &RenderHardware,
    ) -> Result<TaroMeshBinding, BorrowError> {
        if self.matrix.is_none() {
            let buffer = hardware
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[self
                        .transform
                        .data()?
                        .world_matrix()
                        .to_cols_array_2d()]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

            let layout =
                &hardware
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: &[wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        }],
                        label: Some("Model Bind Group Layout"),
                    });

            let binding = hardware
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                    label: Some("camera_bind_group"),
                });

            self.matrix = Some(CompiledMatrixData { buffer, binding });
        }

        let matrix = self.matrix.as_ref().unwrap();

        let mesh = self.mesh.compile(hardware);

        hardware.queue.write_buffer(
            &matrix.buffer,
            0,
            bytemuck::cast_slice(&[self.transform.data()?.world_matrix().to_cols_array_2d()]),
        );

        Ok(TaroMeshBinding {
            mesh,
            matrix: &matrix.binding,
        })
    }
}
