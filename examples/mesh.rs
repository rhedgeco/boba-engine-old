use std::f32::consts::PI;

use boba_3d::pearls::Transform;
use boba_core::*;
use cgmath::Point3;
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
    pub transform: Pearl<Transform>,
    pub offset: f32,
    pub rotation: f32,
    pub speed: f32,
}

impl PearlRegister for CameraRotator {
    fn register(pearl: Pearl<Self>, storage: &mut storage::StageRunners) {
        storage.add(pearl);
    }
}

impl PearlStage<MainBobaUpdate> for CameraRotator {
    fn update(delta: &f32, pearl: &mut Pearl<Self>, _: &mut BobaResources) -> PearlResult {
        let mut pdata = pearl.data_mut()?;

        pdata.rotation = (pdata.rotation + delta * pdata.speed) % (2. * PI);
        let x = pdata.rotation.sin() * pdata.offset;
        let z = pdata.rotation.cos() * pdata.offset;
        let position: Point3<f32> = (x, 1., z).into();

        let mut tdata = pdata.transform.data_mut()?;
        tdata.set_position(position);
        tdata.look_at((0., 0., 0.).into());

        Ok(())
    }
}

fn main() {
    // Create app and renderer with a camera
    env_logger::init();
    let mut app = BobaApp::default();
    app.add_plugin(TaroRenderPlugin);
    let mut renderer = TaroRenderer::default();

    // create and add camera
    let mut camera_transform = Transform::from_position((0., 1., 2.).into());
    camera_transform.look_at((0., 0., 0.).into());
    let camera_transform = camera_transform.as_pearl();
    let camera = TaroCamera::new(
        camera_transform.clone(),
        TaroCameraSettings {
            aspect: 1.,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        },
        renderer.resources(),
    )
    .unwrap()
    .as_pearl();
    renderer.cameras.main_camera = Some(camera);

    // create and add camera rotator
    let rotator = CameraRotator {
        transform: camera_transform,
        offset: 2.,
        rotation: 0.,
        speed: 2.,
    }
    .as_pearl();
    app.stages.add_pearl(rotator);

    // create an arbitrary mesh to show in the center of the screen
    let shader = TaroShader::from_wgsl("Mesh Shader", include_str!("mesh_shader.wgsl")).unwrap();
    let texture =
        TaroTexture::from_bytes("Mesh Texture", include_bytes!("happy-tree.png")).unwrap();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let mesh_renderer = TaroMeshRenderer::new(mesh, shader, texture).as_pearl();
    app.stages.add_pearl(mesh_renderer.clone()); // we clone it so that it can be used later when attaching to renderer
    renderer.pearls.add(mesh_renderer);

    // add the renderer and run the app
    app.resources.add(renderer);
    MilkTeaRunner::run(app).unwrap();
}
