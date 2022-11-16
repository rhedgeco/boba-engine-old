use boba_core::*;
use milk_tea_runner::*;
use taro_renderer::{
    prelude::*,
    renderers::TaroMeshRenderer,
    types::{TaroMesh, TaroShader, TaroTexture, Vertex},
    TaroCamera, TaroCameraSettings, TaroRenderer,
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
    // Create app and renderer with a camera
    env_logger::init();
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    let mut renderer = TaroRenderer::default();
    let camera = Pearl::wrap(TaroCamera::new(
        TaroCameraSettings {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        },
        renderer.resources(),
    ));
    renderer.cameras.main_camera = Some(camera);

    // create an arbitrary mesh to show in the center of the screen
    let shader = TaroShader::from_wgsl("Mesh Shader", include_str!("mesh_shader.wgsl")).unwrap();
    let texture =
        TaroTexture::from_bytes("Mesh Texture", include_bytes!("happy-tree.png")).unwrap();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let mesh_renderer = Pearl::wrap(TaroMeshRenderer::new(mesh, shader, texture));
    app.stages.add_pearl(mesh_renderer.clone()); // we clone it so that it can be used later when attaching to renderer
    renderer.pearls.add(mesh_renderer);

    // add the renderer and run the app
    app.resources.add(renderer);
    MilkTeaRunner::run(app).unwrap();
}
