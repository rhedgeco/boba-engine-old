use boba_core::*;
use boba_mesh::*;
use milk_tea_runner::*;
use taro_renderer::{prelude::*, renderers::*};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [0., 0., 0.], color: [1., 1., 1.] },
    Vertex { position: [-0.5, -0.5, 0.], color: [1., 0., 0.] },
    Vertex { position: [0.5, -0.5, 0.], color: [0., 1., 0.] },
    Vertex { position: [0., 0.707106781, 0.], color: [0., 0., 1.] },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
    0, 3, 1
];

fn main() {
    let mut app = BobaApp::new(MilkTeaRunner::default());
    app.add_plugin(&TaroRenderPlugin);

    let mesh = TaroMesh::new(BobaMesh::new(VERTICES, INDICES));
    let shader_code = include_str!("mesh_shader.wgsl");
    let mesh_renderer = TaroMeshRenderer::new(mesh, shader_code);
    app.controllers().add(BobaController::build(mesh_renderer));

    app.run();
}
