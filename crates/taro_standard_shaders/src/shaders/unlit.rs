use log::error;
use taro_renderer::{
    shading::{TaroMeshShader, TaroShaderCore},
    types::{create_matrix_bind_layout, Vertex},
    wgpu,
};

#[derive(Default)]
pub struct UnlitShader {
    pipeline: Option<wgpu::RenderPipeline>,
}

impl TaroShaderCore for UnlitShader {
    fn prepare(&mut self, resources: &taro_renderer::RenderResources) {
        if self.pipeline.is_none() {
            let module = &resources
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Unlit Shader"),
                    source: taro_renderer::wgpu::ShaderSource::Wgsl(
                        include_str!("unlit.wgsl").into(),
                    ),
                });

            // TODO: potentially find a way to store this instead of create it each time for each shader
            let camera_layout = create_matrix_bind_layout(resources);

            let pipeline_layout =
                resources
                    .device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[&camera_layout, &camera_layout],
                        push_constant_ranges: &[],
                    });

            let render_pipeline =
                resources
                    .device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Unlit Render Pipeline"),
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
        }
    }
}

impl TaroMeshShader for UnlitShader {
    fn render_mesh<'a>(
        &'a self,
        pass: &mut taro_renderer::wgpu::RenderPass<'a>,
        data: &taro_renderer::shading::RenderMeshData<'a>,
    ) {
        let Some(pipeline) = &self.pipeline else {
            error!("Cannot render mesh. Shader is not prepared.");
            return;
        };

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, data.camera_bind_group, &[]);
        pass.set_bind_group(1, data.model_bind_group, &[]);
        pass.set_vertex_buffer(0, data.mesh.vertex_buffer.raw_buffer().slice(..));
        pass.set_index_buffer(
            data.mesh.index_buffer.raw_buffer().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..data.mesh.index_buffer.buffer_length(), 0, 0..1);
    }
}
