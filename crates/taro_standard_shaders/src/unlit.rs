use once_cell::sync::OnceCell;
use taro_renderer::{
    data_types::{
        buffers::{CameraMatrix, TransformMatrix},
        TaroBuffer, UploadedTaroMesh, Vertex,
    },
    shading::{TaroCoreShader, TaroMeshShader},
    wgpu,
};

static PIPELINE: OnceCell<wgpu::RenderPipeline> = OnceCell::new();
static MATRIX_LAYOUT: OnceCell<wgpu::BindGroupLayout> = OnceCell::new();

pub struct UnlitShader {
    _private: (),
}

impl TaroCoreShader for UnlitShader {
    fn build_instance(hardware: &taro_renderer::TaroHardware) -> Self {
        let matrix_layout = MATRIX_LAYOUT.get_or_init(|| {
            hardware
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("Unlit Shader Mat4 Bind Group"),
                })
        });

        PIPELINE.get_or_init(|| {
            let pipeline_layout =
                hardware
                    .device()
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[&matrix_layout, &matrix_layout],
                        push_constant_ranges: &[],
                    });

            let module = &hardware
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Unlit Shader"),
                    source: taro_renderer::wgpu::ShaderSource::Wgsl(
                        include_str!("unlit.wgsl").into(),
                    ),
                });

            hardware
                .device()
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
                            format: wgpu::TextureFormat::Bgra8UnormSrgb,
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
                })
        });

        Self { _private: () }
    }
}

impl TaroMeshShader for UnlitShader {
    fn render<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        mesh: &'pass UploadedTaroMesh,
        camera_matrix: &'pass TaroBuffer<CameraMatrix>,
        model_matrix: &'pass TaroBuffer<TransformMatrix>,
        hardware: &taro_renderer::TaroHardware,
    ) {
        let camera_bind =
            camera_matrix.get_or_init_binding(self, MATRIX_LAYOUT.get().unwrap(), hardware);

        let model_bind =
            model_matrix.get_or_init_binding(self, MATRIX_LAYOUT.get().unwrap(), hardware);

        pass.set_pipeline(PIPELINE.get().unwrap());
        pass.set_bind_group(0, camera_bind, &[]);
        pass.set_bind_group(1, model_bind, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer().raw_buffer().slice(..));
        pass.set_index_buffer(
            mesh.index_buffer().raw_buffer().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..mesh.index_buffer().len(), 0, 0..1);
    }
}
