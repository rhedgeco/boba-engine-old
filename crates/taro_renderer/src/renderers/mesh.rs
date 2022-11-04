use boba_core::*;
use boba_mesh::{BobaMesh, Vertex};
use wgpu::{RenderPass, RenderPipeline, ShaderModuleDescriptor, ShaderSource};

use crate::{stages::TaroRenderStage, TaroRenderer};

pub struct TaroMeshRenderer<'mesh> {
    mesh: BobaMesh<'mesh>,
    shader_code: &'mesh str,
    render_pipeline: Option<RenderPipeline>,
}

impl<'mesh> TaroMeshRenderer<'mesh> {
    pub fn new(mesh: BobaMesh<'mesh>, shader_code: &'mesh str) -> Self {
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
                        buffers: &[Vertex::desc()],
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

        render_pass.set_pipeline(&self.render_pipeline.as_ref().unwrap());
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer().as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.mesh.index_buffer().as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.mesh.index_length(), 0, 0..1);
    }
}

register_controller_with_stages!(TaroMeshRenderer<'mesh>: TaroRenderStage);
