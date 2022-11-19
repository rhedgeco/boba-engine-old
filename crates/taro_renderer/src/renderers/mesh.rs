use boba_core::PearlRegister;
use wgpu::{BindGroup, RenderPass, RenderPipeline};

use crate::{
    types::{CompiledTaroMesh, CompiledTaroTexture, TaroMesh, TaroShader, TaroTexture, Vertex},
    RenderResources, TaroCamera,
};

pub struct TaroMeshRenderer {
    mesh: TaroMesh,
    shader: TaroShader,
    main_texture: TaroTexture,
    pipeline: Option<RenderPipeline>,
}

impl PearlRegister for TaroMeshRenderer {
    fn register(_: boba_core::Pearl<Self>, _: &mut boba_core::storage::StageRunners) {
        // do nothing
    }
}

impl TaroMeshRenderer {
    pub fn new(mesh: TaroMesh, shader: TaroShader, main_texture: TaroTexture) -> Self {
        Self {
            mesh,
            shader,
            main_texture,
            pipeline: None,
        }
    }

    pub fn render_mesh<'a>(
        &'a mut self,
        camera: &'a BindGroup,
        pass: &mut RenderPass<'a>,
        resources: &RenderResources,
    ) {
        let (pipeline, mesh, texture) = self.compile(resources);

        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &texture.bind_group, &[]);
        pass.set_bind_group(1, camera, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer.raw_buffer().slice(..));
        pass.set_index_buffer(
            mesh.index_buffer.raw_buffer().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..mesh.index_buffer.buffer_length(), 0, 0..1);
    }

    fn compile(
        &mut self,
        resources: &RenderResources,
    ) -> (&RenderPipeline, &CompiledTaroMesh, &CompiledTaroTexture) {
        if self.pipeline.is_some() {
            let pipeline = self.pipeline.as_ref().unwrap();
            let mesh = self.mesh.compile(resources);
            let texture = self.main_texture.compile(resources);
            return (&pipeline, &mesh, &texture);
        }

        let texture = self.main_texture.compile(resources);
        let camera_layout = TaroCamera::create_bind_group_layout(resources);

        let pipeline_layout =
            resources
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&texture.bind_group_layout, &camera_layout],
                    push_constant_ranges: &[],
                });

        let module = self.shader.compile(resources);

        let render_pipeline =
            resources
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module,
                        entry_point: "vs_main",
                        buffers: &[Vertex::BUFFER_LAYOUT],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba8UnormSrgb,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None, //Some(wgpu::Face::Back),
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

        self.pipeline = Some(render_pipeline);

        let pipeline = self.pipeline.as_ref().unwrap();
        let mesh = self.mesh.compile(resources);
        return (pipeline, &mesh, &texture);
    }
}
