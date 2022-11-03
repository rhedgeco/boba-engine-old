use boba_core::*;
use boba_mesh::*;
use milk_tea_runner::MilkTeaRunner;
use taro_renderer::{prelude::TaroRenderPlugin, stages::TaroRenderStage, TaroRenderer};
use wgpu::{include_wgsl, RenderPass, RenderPipeline};

struct MeshController<'mesh> {
    render_pipeline: Option<RenderPipeline>,
    mesh: BobaMesh<'mesh>,
}

impl<'mesh> MeshController<'mesh> {
    pub fn new(mesh: BobaMesh<'mesh>) -> Self {
        Self {
            render_pipeline: Default::default(),
            mesh,
        }
    }
}

impl<'mesh> MeshController<'mesh> {
    fn init(&mut self, resources: &mut BobaResources) {
        let renderer = resources.get::<TaroRenderer>().unwrap();

        let shader = renderer
            .device()
            .create_shader_module(include_wgsl!("mesh_shader.wgsl"));

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

impl<'mesh> ControllerStage<TaroRenderStage> for MeshController<'mesh> {
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

register_controller_with_stages!(MeshController<'mesh>: TaroRenderStage);

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

fn main() {
    let mut app = BobaApp::new(MilkTeaRunner::default());
    app.add_plugin(&TaroRenderPlugin);
    let mesh = BobaMesh::new(VERTICES, INDICES);
    app.controllers()
        .add(BobaController::build(MeshController::new(mesh)));
    app.run();
}
