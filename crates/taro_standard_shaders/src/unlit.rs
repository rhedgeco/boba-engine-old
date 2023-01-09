use once_cell::sync::OnceCell;
use taro_renderer::{
    data_types::{TaroMeshBuffer, Vertex},
    shading::{
        buffers::{CameraMatrix, TransformMatrix},
        data_types::{TaroSampler, Texture2DView},
        BindGroup, BindGroupBuilder, TaroBinding, TaroCoreShader, TaroMeshShader, UniformBuffer,
    },
    wgpu, TaroHardware,
};

static PIPELINE: OnceCell<wgpu::RenderPipeline> = OnceCell::new();
static MATRIX_LAYOUT: OnceCell<wgpu::BindGroupLayout> = OnceCell::new();

pub struct UnlitShaderInit {
    pub view: Texture2DView,
    pub sampler: TaroSampler,
}

impl UnlitShaderInit {
    pub fn new(view: Texture2DView, sampler: TaroSampler) -> Self {
        Self { view, sampler }
    }
}

pub struct UnlitShader {
    pub texture: BindGroup,
    pub camera_matrix: TaroBinding<UniformBuffer<CameraMatrix>>,
    pub model_matrix: TaroBinding<UniformBuffer<TransformMatrix>>,
    _pivate: (),
}

impl TaroCoreShader for UnlitShader {
    type InitParameters = UnlitShaderInit;

    fn build_instance(init: &UnlitShaderInit, hardware: &TaroHardware) -> Self {
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

        let view_bind = TaroBinding::new(init.view.clone(), wgpu::ShaderStages::FRAGMENT);
        let sampler_bind = TaroBinding::new(init.sampler.clone(), wgpu::ShaderStages::FRAGMENT);
        let texture = BindGroupBuilder::new()
            .set_binding(0, view_bind)
            .set_binding(1, sampler_bind)
            .build();

        PIPELINE.get_or_init(|| {
            let pipeline_layout =
                hardware
                    .device()
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[
                            &matrix_layout,
                            &matrix_layout,
                            &texture.get_or_compile(hardware).layout,
                        ],
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

        Self {
            texture,
            camera_matrix: TaroBinding::new(UniformBuffer::new(), wgpu::ShaderStages::VERTEX),
            model_matrix: TaroBinding::new(UniformBuffer::new(), wgpu::ShaderStages::VERTEX),
            _pivate: (),
        }
    }
}

impl TaroMeshShader for UnlitShader {
    fn set_camera_matrix(&self, data: &CameraMatrix, hardware: &TaroHardware) {
        self.camera_matrix
            .binding_data()
            .write_buffer(data, hardware);
    }

    fn set_model_matrix(&self, data: &TransformMatrix, hardware: &TaroHardware) {
        self.model_matrix
            .binding_data()
            .write_buffer(data, hardware);
    }

    fn render<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        mesh: &'pass TaroMeshBuffer,
        hardware: &TaroHardware,
    ) {
        let camera_bind = self.camera_matrix.get_or_compile_single(hardware);
        let model_bind = self.model_matrix.get_or_compile_single(hardware);
        let texture_bind = self.texture.get_or_compile(hardware);

        pass.set_pipeline(PIPELINE.get().unwrap());
        pass.set_bind_group(0, &camera_bind.bind_group, &[]);
        pass.set_bind_group(1, &model_bind.bind_group, &[]);
        pass.set_bind_group(2, &texture_bind.bind_group, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer().raw_buffer().slice(..));
        pass.set_index_buffer(
            mesh.index_buffer().raw_buffer().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..mesh.index_buffer().len(), 0, 0..1);
    }
}
