use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{
    prelude::*,
    renderers::TaroMeshRenderer,
    types::{TaroMesh, TaroShader, TaroTexture, Vertex},
    TaroCamera, TaroRenderer,
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

struct CameraRotator;

impl BobaController for CameraRotator {}

impl BobaUpdate<MainBobaUpdate> for CameraRotator {
    fn update(&mut self, data: &(), resources: &mut BobaResources) {
        let Ok(camera) = resources.borrow_mut::<TaroCamera>() else {
            return;
        };
    }
}

fn main() {
    env_logger::init();
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);

    let shader = TaroShader::from_wgsl("Mesh Shader", include_str!("mesh_shader.wgsl")).unwrap();
    let texture =
        TaroTexture::from_bytes("Mesh Texture", include_bytes!("happy-tree.png")).unwrap();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let mesh_renderer = BobaContainer::build(TaroMeshRenderer::new(mesh, shader, texture));
    app.stages().add_controller(mesh_renderer.clone());

    let mut renderer = TaroRenderer::default();
    renderer.render_controllers().add(mesh_renderer);
    app.resources().add(renderer);
    MilkTeaRunner::run(app).unwrap();
}
