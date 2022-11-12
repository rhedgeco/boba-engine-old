use log::warn;
use wgpu::{util::DeviceExt, Buffer};

use crate::TaroRenderer;

use super::TaroCompiler;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

pub struct TaroBuffer {
    raw_buffer: Buffer,
    length: u32,
}

impl TaroBuffer {
    pub fn raw_buffer(&self) -> &Buffer {
        &self.raw_buffer
    }

    pub fn buffer_length(&self) -> u32 {
        self.length
    }
}

pub struct CompiledTaroMesh {
    pub vertex_buffer: TaroBuffer,
    pub index_buffer: TaroBuffer,
}

pub struct TaroMesh<'a> {
    vertices: &'a [Vertex],
    indices: &'a [u16],
    buffers: Option<CompiledTaroMesh>,
}

impl<'a> TaroMesh<'a> {
    pub const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
    };

    pub fn new(vertices: &'a [Vertex], indices: &'a [u16]) -> Self {
        Self {
            vertices,
            indices,
            buffers: None,
        }
    }
}

impl<'a> TaroCompiler for TaroMesh<'a> {
    type CompiledData = CompiledTaroMesh;

    fn get_data(&self) -> &Option<Self::CompiledData> {
        &self.buffers
    }

    fn compile(&mut self, renderer: &TaroRenderer) {
        if self.buffers.is_some() {
            return;
        }

        let Some(render_resources) = renderer.resources() else {
            warn!("Could not compile/upload mesh. TaroRenderer has not been initialized");
            return;
        };

        self.buffers = Some(CompiledTaroMesh {
            vertex_buffer: TaroBuffer {
                length: self.vertices.len() as u32,
                raw_buffer: render_resources.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(self.vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    },
                ),
            },
            index_buffer: TaroBuffer {
                length: self.indices.len() as u32,
                raw_buffer: render_resources.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(self.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    },
                ),
            },
        });
    }
}
