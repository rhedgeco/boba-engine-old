use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{
    prelude::*,
    renderers::TaroMeshRenderer,
    types::{TaroMesh, TaroShader, TaroTexture, Vertex},
};

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
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);

    let shader = TaroShader::from_str(Some("Mesh Shader"), include_str!("mesh_shader.wgsl"));
    let texture =
        TaroTexture::from_bytes(Some("Mesh Texture"), include_bytes!("happy-tree.png")).unwrap();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let renderer = TaroMeshRenderer::new(mesh, shader, texture);
    app.controllers().add(BobaController::build(renderer));

    MilkTeaRunner::run(app).unwrap();
}