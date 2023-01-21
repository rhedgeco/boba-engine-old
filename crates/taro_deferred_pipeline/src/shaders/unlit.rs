use std::sync::Arc;

use once_map::OnceMap;
use taro_core::{
    data::{
        buffers::{CameraMatrix, TransformMatrix},
        texture::{Simple, Texture2DView},
        Mesh, Sampler, UniformBinding, Vertex,
    },
    wgpu::{self, RenderPass},
    Bind, BindGroup, BindGroupBuilder, HardwareId, Taro, TaroHardware,
};

use super::DeferredShader;

pub struct UnlitShader {
    texture: Taro<BindGroup>,
}

impl UnlitShader {
    pub fn new(texture: Taro<Texture2DView<Simple>>) -> Arc<Self> {
        let sampler = Bind::new(Sampler::new());
        let texture = BindGroupBuilder::new(0, Bind::new(texture))
            .insert(1, sampler)
            .build();

        Arc::new(Self { texture })
    }
}

impl DeferredShader for UnlitShader {
    fn render_gbuffer_albedo<'pass>(
        &'pass self,
        mesh: &'pass Taro<Mesh>,
        camera_matrix: &'pass Taro<UniformBinding<CameraMatrix>>,
        model_matrix: &'pass Taro<UniformBinding<TransformMatrix>>,
        pass: &mut RenderPass<'pass>,
        hardware: &TaroHardware,
    ) {
        let mesh_buffer = mesh.get_or_compile(hardware);
        let texture_binding = self.texture.get_or_compile(hardware);
        let camera_binding = camera_matrix.get_or_compile(hardware);
        let model_binding = model_matrix.get_or_compile(hardware);

        static PIPELINE: OnceMap<HardwareId, wgpu::RenderPipeline> = OnceMap::new();
        let pipeline = PIPELINE
            .get_or_init(*hardware.id(), || {
                let layout =
                    hardware
                        .device()
                        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                            label: Some("Render Pipeline Layout"),
                            bind_group_layouts: &[
                                camera_binding.layout(),
                                model_binding.layout(),
                                texture_binding.layout(),
                            ],
                            push_constant_ranges: &[],
                        });

                let module =
                    &hardware
                        .device()
                        .create_shader_module(wgpu::ShaderModuleDescriptor {
                            label: Some("Unlit Shader"),
                            source: wgpu::ShaderSource::Wgsl(
                                include_str!("unlit_albedo.wgsl").into(),
                            ),
                        });

                hardware
                    .device()
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Unlit Render Pipeline"),
                        layout: Some(&layout),
                        vertex: wgpu::VertexState {
                            module,
                            entry_point: "vs_main",
                            buffers: &[Vertex::BUFFER_LAYOUT],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module,
                            entry_point: "fs_main",
                            targets: &[Some(wgpu::ColorTargetState {
                                format: wgpu::TextureFormat::Bgra8UnormSrgb,
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
                        depth_stencil: Some(wgpu::DepthStencilState {
                            format: wgpu::TextureFormat::Depth32Float,
                            depth_write_enabled: true,
                            depth_compare: wgpu::CompareFunction::Less,
                            stencil: wgpu::StencilState::default(),
                            bias: wgpu::DepthBiasState::default(),
                        }),
                        multisample: wgpu::MultisampleState {
                            count: 1,
                            mask: !0,
                            alpha_to_coverage_enabled: false,
                        },
                        multiview: None,
                    })
            })
            .into_data();

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, camera_binding.bind_group(), &[]);
        pass.set_bind_group(1, model_binding.bind_group(), &[]);
        pass.set_bind_group(2, texture_binding.bind_group(), &[]);
        pass.set_vertex_buffer(0, mesh_buffer.vertex_buffer().raw_buffer().slice(..));
        pass.set_index_buffer(
            mesh_buffer.index_buffer().raw_buffer().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..mesh_buffer.index_buffer().len(), 0, 0..1);
    }
}
