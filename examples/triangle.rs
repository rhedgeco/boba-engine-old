use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{prelude::TaroRenderPlugin, stages::TaroRenderStage, TaroRenderer};
use wgpu::{include_wgsl, RenderPass, RenderPipeline};

#[derive(Default)]
struct TriangleController {
    render_pipeline: Option<RenderPipeline>,
}

impl TriangleController {
    fn init(&mut self, resources: &mut BobaResources) {
        let renderer = resources.get::<TaroRenderer>().unwrap();

        let shader = renderer
            .device()
            .create_shader_module(include_wgsl!("triangle_shader.wgsl"));

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
                        buffers: &[],
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
    }
}

impl ControllerStage<TaroRenderStage> for TriangleController {
    fn update<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, resources: &mut BobaResources) {
        if self.render_pipeline.is_none() {
            self.init(resources);
        }

        render_pass.set_pipeline(&self.render_pipeline.as_ref().unwrap());
        render_pass.draw(0..3, 0..1);
    }
}

register_controller_with_stages!(TriangleController: TaroRenderStage);

fn main() {
    let mut app = BobaApp::new(MilkTeaRunner::default());
    app.add_plugin(&TaroRenderPlugin);
    app.controllers()
        .add(BobaController::build(TriangleController::default()));
    app.run();
}
