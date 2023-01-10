use std::{
    fmt::Debug,
    fs::File,
    io::BufReader,
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

use tobj::LoadError;
use wgpu::util::DeviceExt;

use crate::{
    shading::{TaroData, TaroDataUploader},
    HardwareId, TaroHardware,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3],
    };
}

pub struct MeshData<T> {
    raw_buffer: wgpu::Buffer,
    length: AtomicU32,
    _type: PhantomData<T>,
}

impl<T> MeshData<T> {
    pub fn len(&self) -> u32 {
        self.length.load(Ordering::Relaxed)
    }

    pub fn raw_buffer(&self) -> &wgpu::Buffer {
        &self.raw_buffer
    }
}

impl MeshData<Vertex> {
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

impl MeshData<u16> {
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
    vertex_buffer: MeshData<Vertex>,
    index_buffer: MeshData<u16>,
}

impl TaroMeshBuffer {
    pub fn hardware_id(&self) -> &HardwareId {
        &self.hardware_id
    }

    pub fn vertex_buffer(&self) -> &MeshData<Vertex> {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &MeshData<u16> {
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
    pub fn new(obj_file: File) -> Result<Self, LoadError> {
        let mut reader = BufReader::new(obj_file);
        let (models, _) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                single_index: false,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
            },
            |_| Ok(Default::default()),
        )?;

        let mesh = &models[0].mesh;
        let mut vertices = Vec::<Vertex>::new();
        let mut indices = Vec::<u16>::new();

        for i in 0..mesh.indices.len() / 3 {
            let i1 = i;
            let i2 = i + 1;
            let i3 = i + 2;

            indices.push(i as u16);
            vertices.push(Vertex {
                position: [mesh.positions[i1], mesh.positions[i2], mesh.positions[i3]],
                uv: [0., 0.],
                normal: [0., 0., 0.],
            });
        }

        Ok(Self::from_vertices(&vertices, &indices))
    }

    pub fn from_vertices(vertices: &[Vertex], indices: &[u16]) -> Self {
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
            vertex_buffer: MeshData::<Vertex>::new(&self.vertices, hardware),
            index_buffer: MeshData::<u16>::new(&self.indices, hardware),
        }
    }
}
