use wgpu::{util::DeviceExt, Buffer};

use crate::RenderResources;

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

pub struct TaroMesh {
    vertices: Box<[Vertex]>,
    indices: Box<[u16]>,
    compiled: Option<CompiledTaroMesh>,
}

impl TaroMesh {
    pub const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
    };

    pub fn new(vertices: &[Vertex], indices: &[u16]) -> Self {
        Self {
            vertices: Box::<[Vertex]>::from(vertices),
            indices: Box::<[u16]>::from(indices),
            compiled: None,
        }
    }

    pub fn compile(&mut self, resources: &RenderResources) -> &CompiledTaroMesh {
        if self.compiled.is_some() {
            return self.compiled.as_ref().unwrap();
        }

        self.compiled = Some(CompiledTaroMesh {
            vertex_buffer: TaroBuffer {
                length: self.vertices.len() as u32,
                raw_buffer: resources.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(self.vertices.as_ref()),
                        usage: wgpu::BufferUsages::VERTEX,
                    },
                ),
            },
            index_buffer: TaroBuffer {
                length: self.indices.len() as u32,
                raw_buffer: resources.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(self.indices.as_ref()),
                        usage: wgpu::BufferUsages::INDEX,
                    },
                ),
            },
        });

        self.compiled.as_ref().unwrap()
    }
}
