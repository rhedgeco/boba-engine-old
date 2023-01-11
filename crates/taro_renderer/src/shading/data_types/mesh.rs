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
    shading::{Taro, TaroBuilder},
    TaroHardware,
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

pub struct MeshBuffer {
    vertex_buffer: MeshData<Vertex>,
    index_buffer: MeshData<u16>,
}

impl MeshBuffer {
    pub fn vertex_buffer(&self) -> &MeshData<Vertex> {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &MeshData<u16> {
        &self.index_buffer
    }
}

pub struct Mesh {
    vertices: Box<[Vertex]>,
    indices: Box<[u16]>,
}

impl Mesh {
    pub fn new(obj_file: File) -> Result<Taro<Self>, LoadError> {
        let mut reader = BufReader::new(obj_file);
        let (models, _) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
            },
            |_| Ok(Default::default()),
        )?;

        let mut vertices = Vec::<Vertex>::new();
        let mut indices = Vec::<u16>::new();

        for model in models {
            let mesh = model.mesh;
            for index in &mesh.indices {
                let pos_offset = (3 * index) as usize;
                let texcoord_offset = (2 * index) as usize;

                let vertex = Vertex {
                    position: [
                        mesh.positions[pos_offset + 0],
                        mesh.positions[pos_offset + 1],
                        mesh.positions[pos_offset + 2],
                    ],
                    uv: [
                        mesh.texcoords[texcoord_offset + 0],
                        1. - mesh.texcoords[texcoord_offset + 1],
                    ],
                    normal: [0., 0., 0.],
                };

                vertices.push(vertex);
                indices.push(indices.len() as u16);
            }
        }

        Ok(Self::from_vertices(&vertices, &indices))
    }

    pub fn from_vertices(vertices: &[Vertex], indices: &[u16]) -> Taro<Self> {
        Taro::new(Self {
            vertices: Box::<[Vertex]>::from(vertices),
            indices: Box::<[u16]>::from(indices),
        })
    }
}

impl TaroBuilder for Mesh {
    type Compiled = MeshBuffer;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        MeshBuffer {
            vertex_buffer: MeshData::<Vertex>::new(&self.vertices, hardware),
            index_buffer: MeshData::<u16>::new(&self.indices, hardware),
        }
    }
}

impl Taro<Mesh> {
    pub fn write_vertices(&self, vertices: &[Vertex], hardware: &TaroHardware) {
        let buffer = self.get_or_compile(hardware);
        buffer.vertex_buffer.write(vertices, hardware);
    }

    pub fn write_indices(&self, indices: &[u16], hardware: &TaroHardware) {
        let buffer = self.get_or_compile(hardware);
        buffer.index_buffer.write(indices, hardware);
    }
}
