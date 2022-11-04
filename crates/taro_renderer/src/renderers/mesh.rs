use boba_core::*;
use boba_mesh::{BobaMesh, Vertex};
use wgpu::{util::DeviceExt, RenderPass, RenderPipeline, ShaderModuleDescriptor, ShaderSource};

use crate::{stages::TaroRenderStage, TaroRenderer};

const VERTEX_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
};

struct TaroMeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

pub struct TaroMesh<'mesh> {
    mesh: BobaMesh<'mesh>,
    buffers: Option<TaroMeshBuffers>,
}

impl<'mesh> TaroMesh<'mesh> {
    pub fn new(mesh: BobaMesh<'mesh>) -> Self {
        Self {
            mesh,
            buffers: None,
        }
    }

    pub fn index_length(&self) -> u32 {
        self.mesh.index_length()
    }

    pub fn upload(&mut self, device: &wgpu::Device) {
        self.buffers = Some(TaroMeshBuffers {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(self.mesh.vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(self.mesh.indices()),
                usage: wgpu::BufferUsages::INDEX,
            }),
        })
    }
}

pub struct TaroMeshRenderer<'mesh> {
    mesh: TaroMesh<'mesh>,
    shader_code: &'mesh str,
    render_pipeline: Option<RenderPipeline>,
}

impl<'mesh> TaroMeshRenderer<'mesh> {
    pub fn new(mesh: TaroMesh<'mesh>, shader_code: &'mesh str) -> Self {
        Self {
            mesh,
            shader_code,
            render_pipeline: None,
        }
    }

    fn init(&mut self, resources: &mut BobaResources) {
        let renderer = resources.get::<TaroRenderer>().unwrap();

        let shader = renderer
            .device()
            .create_shader_module(ShaderModuleDescriptor {
                label: Some("Shader"),
                source: ShaderSource::Wgsl(self.shader_code.into()),
            });

        let render_pipeline_layout =
            renderer
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            renderer
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[VERTEX_LAYOUT],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: renderer.config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        self.render_pipeline = Some(render_pipeline);
        self.mesh.upload(renderer.device());
    }
}

impl<'mesh> ControllerStage<TaroRenderStage> for TaroMeshRenderer<'mesh> {
    fn update<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, resources: &mut BobaResources) {
        if self.render_pipeline.is_none() {
            self.init(resources);
        }

        let buffers = self
            .mesh
            .buffers
            .as_ref()
            .expect("Buffers should be uploaded by this point");

        render_pass.set_pipeline(&self.render_pipeline.as_ref().unwrap());
        render_pass.set_vertex_buffer(0, buffers.vertex_buffer.slice(..));
        render_pass.set_index_buffer(buffers.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.mesh.index_length(), 0, 0..1);
    }
}

register_controller_with_stages!(TaroMeshRenderer<'mesh>: TaroRenderStage);
