use boba_core::{BobaResources, BobaUpdate};
use wgpu::RenderPipeline;

use crate::{
    stages::OnTaroRender,
    types::{TaroCompiler, TaroMesh, TaroShader, TaroTexture},
    RenderResources, TaroCamera, TaroRenderer,
};

pub struct TaroMeshPipelineData {
    pub render_pipeline: RenderPipeline,
}

pub struct TaroMeshRenderer {
    mesh: TaroMesh,
    shader: TaroShader,
    main_texture: TaroTexture,
    pipeline: Option<TaroMeshPipelineData>,
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

    pub fn mesh(&self) -> &TaroMesh {
        &self.mesh
    }

    pub fn texture(&self) -> &TaroTexture {
        &self.main_texture
    }

    pub fn pipeline(&self) -> &Option<TaroMeshPipelineData> {
        &self.pipeline
    }

    pub fn compiled(&self) -> bool {
        self.pipeline.is_some()
    }

    pub fn precompile(&mut self, resources: &RenderResources) {
        if self.pipeline.is_some() {
            return;
        }

        let texture = self.main_texture.get_compiled_data(resources);
        let camera_bind_group_layout = TaroCamera::create_bind_group_layout(resources);

        let pipeline_layout =
            resources
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&texture.bind_group_layout, &camera_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let shader = self.shader.compile(resources);

        let render_pipeline =
            resources
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader.module,
                        entry_point: "vs_main",
                        buffers: &[TaroMesh::VERTEX_LAYOUT],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader.module,
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

        self.pipeline = Some(TaroMeshPipelineData { render_pipeline });

        self.mesh.compile(resources);
    }
}

impl BobaUpdate<OnTaroRender> for TaroMeshRenderer {
    fn update(&mut self, _: &(), resources: &mut BobaResources) {
        if self.compiled() {
            return;
        }

        if let Ok(renderer) = resources.borrow::<TaroRenderer>() {
            self.precompile(renderer.resources())
        }
    }
}
