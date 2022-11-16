use boba_core::*;
use cgmath::{Quaternion, Rotation};
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

struct CameraRotator {
    camera: BobaContainer<TaroCamera>,
    rotation: f32,
}

impl CameraRotator {
    pub fn new(camera: BobaContainer<TaroCamera>) -> Self {
        Self {
            camera,
            rotation: 1.,
        }
    }
}

impl BobaController for CameraRotator {}

impl BobaUpdate<MainBobaUpdate> for CameraRotator {
    fn update(&mut self, delta: &f32, _: &mut BobaResources) {
        let rotation = Quaternion::from_sv(1., (0., self.rotation * delta, 0.).into());
        let mut camera = self.camera.data().borrow_mut();
        camera.settings.eye = rotation.rotate_point(camera.settings.eye);
    }
}

fn main() {
    // Create app and renderer with a camera
    env_logger::init();
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    let mut renderer = TaroRenderer::default();
    let camera = BobaContainer::build(TaroCamera::new(
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
    renderer.cameras.main_camera = Some(camera.clone()); // we clone it so that it can be used later

    // create the rotator and attach the camera to it
    let rotator = BobaContainer::build(CameraRotator::new(camera));
    app.stages.add_controller(rotator);

    // create an arbitrary mesh to show in the center of the screen
    let shader = TaroShader::from_wgsl("Mesh Shader", include_str!("mesh_shader.wgsl")).unwrap();
    let texture =
        TaroTexture::from_bytes("Mesh Texture", include_bytes!("happy-tree.png")).unwrap();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let mesh_renderer = BobaContainer::build(TaroMeshRenderer::new(mesh, shader, texture));
    app.stages.add_controller(mesh_renderer.clone()); // we clone it so that it can be used later
    renderer.controllers.add(mesh_renderer);

    // add the renderer and run the app
    app.resources.add(renderer);
    MilkTeaRunner::run(app).unwrap();
}
