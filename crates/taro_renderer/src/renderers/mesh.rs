use boba_core::{BobaController, BobaResources, BobaUpdate};
use log::warn;
use wgpu::{BindGroup, BindGroupLayoutDescriptor, RenderPipeline};

use crate::{
    stages::OnTaroRender,
    types::{TaroCompiler, TaroMesh, TaroShader, TaroTexture},
    TaroRenderer,
};

pub struct TaroMeshPipelineData {
    pub bind_group: BindGroup,
    pub render_pipeline: RenderPipeline,
}

pub struct TaroMeshRenderer<'a> {
    mesh: TaroMesh,
    shader: TaroShader<'a>,
    main_texture: TaroTexture<'a>,
    pipeline: Option<TaroMeshPipelineData>,
}

impl<'a> TaroMeshRenderer<'a> {
    pub fn new(mesh: TaroMesh, shader: TaroShader<'a>, main_texture: TaroTexture<'a>) -> Self {
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

    pub fn pipeline(&self) -> &Option<TaroMeshPipelineData> {
        &self.pipeline
    }

    pub fn compiled(&self) -> bool {
        self.pipeline.is_some()
    }

    pub fn precompile(&mut self, renderer: &TaroRenderer) {
        if self.pipeline.is_some() {
            return;
        }

        let Some(render_resources) = renderer.resources() else {
            warn!("Could not compile TaroMeshRenderer. TaroRenderer is not initialized.");
            return;
        };

        let texture = self.main_texture.get_compiled_data(renderer);

        let bind_group_layout =
            render_resources
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Render Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            // This should match the filterable field of the
                            // corresponding Texture entry above.
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let sampler = render_resources
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

        let bind_group = render_resources
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Render Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

        let pipeline_layout =
            render_resources
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let shader = self.shader.get_compiled_data(renderer);

        let render_pipeline =
            render_resources
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader.module,
                        entry_point: "vs_main",              // 1.
                        buffers: &[TaroMesh::VERTEX_LAYOUT], // 2.
                    },
                    fragment: Some(wgpu::FragmentState {
                        // 3.
                        module: &shader.module,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            // 4.
                            format: render_resources.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw, // 2.
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None, // 1.
                    multisample: wgpu::MultisampleState {
                        count: 1,                         // 2.
                        mask: !0,                         // 3.
                        alpha_to_coverage_enabled: false, // 4.
                    },
                    multiview: None, // 5.
                });

        self.pipeline = Some(TaroMeshPipelineData {
            bind_group,
            render_pipeline,
        });

        self.mesh.compile(renderer);
    }
}

impl BobaController for TaroMeshRenderer<'_> {}

impl BobaUpdate<OnTaroRender> for TaroMeshRenderer<'_> {
    fn update(&mut self, _: &(), resources: &mut BobaResources) {
        if self.compiled() {
            return;
        }

        if let Ok(renderer) = resources.borrow::<TaroRenderer>() {
            self.precompile(&*renderer);
        }
    }
}
