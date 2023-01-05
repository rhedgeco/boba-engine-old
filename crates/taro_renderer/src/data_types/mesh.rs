use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

use wgpu::util::DeviceExt;

use crate::{
    shading::{TaroData, TaroDataUploader},
    HardwareId, TaroHardware,
};

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

pub struct MeshDataBuffer<T> {
    raw_buffer: wgpu::Buffer,
    length: AtomicU32,
    _type: PhantomData<T>,
}

impl<T> MeshDataBuffer<T> {
    pub fn len(&self) -> u32 {
        self.length.load(Ordering::Relaxed)
    }

    pub fn raw_buffer(&self) -> &wgpu::Buffer {
        &self.raw_buffer
    }
}

impl MeshDataBuffer<Vertex> {
    fn new(vertices: &[Vertex], hardware: &TaroHardware) -> Self {
        Self {
            _type: Default::default(),
            length: AtomicU32::new(vertices.len() as u32),
            raw_buffer: hardware
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
        }
    }

    fn write(&self, vertices: &[Vertex], hardware: &TaroHardware) {
        hardware
            .queue()
            .write_buffer(&self.raw_buffer, 0, bytemuck::cast_slice(vertices));
        self.length.store(vertices.len() as u32, Ordering::Relaxed);
    }
}

impl MeshDataBuffer<u16> {
    fn new(indices: &[u16], hardware: &TaroHardware) -> Self {
        Self {
            _type: Default::default(),
            length: AtomicU32::new(indices.len() as u32),
            raw_buffer: hardware
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                }),
        }
    }

    fn write(&self, indices: &[u16], hardware: &TaroHardware) {
        hardware
            .queue()
            .write_buffer(&self.raw_buffer, 0, bytemuck::cast_slice(indices));
        self.length.store(indices.len() as u32, Ordering::Relaxed);
    }
}

pub struct TaroMeshBuffer {
    hardware_id: HardwareId,
    vertex_buffer: MeshDataBuffer<Vertex>,
    index_buffer: MeshDataBuffer<u16>,
}

impl TaroMeshBuffer {
    pub fn hardware_id(&self) -> &HardwareId {
        &self.hardware_id
    }

    pub fn vertex_buffer(&self) -> &MeshDataBuffer<Vertex> {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &MeshDataBuffer<u16> {
        &self.index_buffer
    }
}

impl TaroData<TaroMesh> for TaroMeshBuffer {
    fn write_new(&self, new_data: &TaroMesh, hardware: &TaroHardware) {
        self.vertex_buffer.write(&new_data.vertices, hardware);
        self.index_buffer.write(&new_data.indices, hardware);
    }
}

pub struct TaroMesh {
    vertices: Box<[Vertex]>,
    indices: Box<[u16]>,
}

impl TaroMesh {
    pub fn new(vertices: &[Vertex], indices: &[u16]) -> Self {
        Self {
            vertices: Box::<[Vertex]>::from(vertices),
            indices: Box::<[u16]>::from(indices),
        }
    }
}

impl TaroDataUploader for TaroMesh {
    type UploadData = TaroMeshBuffer;

    fn new_upload(&self, hardware: &TaroHardware) -> Self::UploadData {
        TaroMeshBuffer {
            hardware_id: hardware.id().clone(),
            vertex_buffer: MeshDataBuffer::<Vertex>::new(&self.vertices, hardware),
            index_buffer: MeshDataBuffer::<u16>::new(&self.indices, hardware),
        }
    }
}
