use wgpu::util::DeviceExt;

use crate::Vertex;

pub struct BobaMesh<'mesh> {
    vertices: &'mesh [Vertex],
    indices: &'mesh [u16],
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    index_length: u32,
}

impl<'a> BobaMesh<'a> {
    pub fn new(vertices: &'a [Vertex], indices: &'a [u16]) -> Self {
        Self {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
            index_length: indices.len() as u32,
        }
    }

    pub fn vertex_buffer(&self) -> &Option<wgpu::Buffer> {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Option<wgpu::Buffer> {
        &self.index_buffer
    }

    pub fn index_length(&self) -> u32 {
        self.index_length
    }

    pub fn upload(&mut self, device: &wgpu::Device) {
        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );

        self.index_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
    }
}
