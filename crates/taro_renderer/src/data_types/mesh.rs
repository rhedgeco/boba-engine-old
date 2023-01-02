use once_map::OnceMap;
use wgpu::util::DeviceExt;

use crate::{HardwareId, TaroHardware};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
    };
}

pub struct MeshBuffer {
    pub raw_buffer: wgpu::Buffer,
    pub length: u32,
}

pub struct UploadedTaroMesh {
    hardware_id: HardwareId,
    vertex_buffer: MeshBuffer,
    index_buffer: MeshBuffer,
}

impl UploadedTaroMesh {
    pub fn hardware_id(&self) -> &HardwareId {
        &self.hardware_id
    }

    pub fn vertex_buffer(&self) -> &MeshBuffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &MeshBuffer {
        &self.index_buffer
    }
}

pub struct TaroMesh {
    vertices: Box<[Vertex]>,
    indices: Box<[u16]>,
    mesh_cache: OnceMap<HardwareId, UploadedTaroMesh>,
}

impl TaroMesh {
    pub fn new(vertices: &[Vertex], indices: &[u16]) -> Self {
        Self {
            vertices: Box::<[Vertex]>::from(vertices),
            indices: Box::<[u16]>::from(indices),
            mesh_cache: Default::default(),
        }
    }

    pub fn upload(&self, hardware: &TaroHardware) -> &UploadedTaroMesh {
        self.mesh_cache
            .get_or_init(hardware.id(), || UploadedTaroMesh {
                hardware_id: hardware.id().clone(),
                vertex_buffer: MeshBuffer {
                    length: self.vertices.len() as u32,
                    raw_buffer: hardware.device().create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&self.vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    ),
                },
                index_buffer: MeshBuffer {
                    length: self.indices.len() as u32,
                    raw_buffer: hardware.device().create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(&self.indices),
                            usage: wgpu::BufferUsages::INDEX,
                        },
                    ),
                },
            })
    }
}
