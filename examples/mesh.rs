use boba_core::*;
use boba_mesh::*;
use milk_tea_runner::*;
use taro_renderer::{prelude::*, renderers::*, TaroTexture};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.], color: [1., 1., 1.], uv: [0., 1.] },
    Vertex { position: [0.5, -0.5, 0.], color: [1., 0., 0.], uv: [1., 1.] },
    Vertex { position: [0.5, 0.5, 0.], color: [0., 1., 0.], uv: [1., 0.] },
    Vertex { position: [-0.5, 0.5, 0.], color: [0., 0., 1.], uv: [0., 0.] },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];

fn main() {
    let mut app = BobaApp::new(MilkTeaRunner::default());
    app.add_plugin(&TaroRenderPlugin);

    let mesh = TaroMesh::new(BobaMesh::new(VERTICES, INDICES));
    let shader_code = include_str!("mesh_shader.wgsl");

    let mut mesh_renderer = TaroMeshRenderer::new(mesh, shader_code);
    let texture = TaroTexture::new(include_bytes!("happy-tree.png"));
    mesh_renderer.set_main_texture(texture);
    app.controllers().add(BobaController::build(mesh_renderer));

    app.run();
}
