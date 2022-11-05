use wgpu::{util::DeviceExt, Buffer};

use super::TaroCompiler;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

pub struct CompiledTaroMesh {
    pub(self) vertex_buffer: Buffer,
    pub(self) index_buffer: Buffer,
    pub(self) vertex_count: u32,
    pub(self) index_count: u32,
}

impl CompiledTaroMesh {
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
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

    fn compile(&mut self, renderer: &crate::TaroRenderer) {
        if self.buffers.is_some() {
            return;
        }

        self.buffers = Some(CompiledTaroMesh {
            vertex_buffer: renderer.device().create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(self.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ),
            index_buffer: renderer
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(self.indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
            vertex_count: self.vertices.len() as u32,
            index_count: self.indices.len() as u32,
        });
    }
}
