use std::f32::consts::PI;

use boba_3d::pearls::BobaTransform;
use boba_core::*;
use glam::{Quat, Vec3};
use milk_tea_runner::*;
use taro_pbr::{phases::UnlitRenderPhase, shaders::UnlitShader};
use taro_renderer::{
    prelude::*,
    renderers::TaroMeshRenderer,
    shading::WrapShader,
    types::{TaroMesh, Vertex},
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

struct TransformRotator {
    pub transform: Pearl<BobaTransform>,
    pub rotation: f32,
    pub speed: f32,
}

impl PearlRegister for TransformRotator {
    fn register(pearl: Pearl<Self>, storage: &mut storage::StageRunners) {
        storage.add(pearl);
    }
}

impl PearlStage<MainBobaUpdate> for TransformRotator {
    fn update(delta: &f32, pearl: &mut Pearl<Self>, _: &mut BobaResources) -> PearlResult {
        let mut pdata = pearl.data_mut()?;
        pdata.rotation = (pdata.rotation + delta * pdata.speed) % (2. * PI);

        let mut tdata = pdata.transform.data_mut()?;
        tdata.set_local_rotation(Quat::from_axis_angle(Vec3::Y, pdata.rotation));

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
    let mut camera_transform = BobaTransform::from_position(Vec3::new(0., 1., 2.));
    camera_transform.look_at(Vec3::ZERO);
    let camera_transform = camera_transform.as_pearl();
    let mut camera = TaroCamera::new(
        camera_transform.clone(),
        TaroCameraSettings {
            aspect: 1.,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        },
        renderer.resources(),
    )
    .unwrap();
    camera.phases.add(UnlitRenderPhase);
    let camera = camera.as_pearl();
    renderer.cameras.main_camera = Some(camera);

    // create an arbitrary mesh to show in the center of the screen
    let model_transform = BobaTransform::from_position(Vec3::ZERO).as_pearl();
    app.stages.add_pearl(model_transform.clone());
    let shader = UnlitShader::default().wrap();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let mesh_renderer =
        TaroMeshRenderer::new(model_transform.clone(), mesh, shader.id()).as_pearl();
    app.stages.add_pearl(mesh_renderer.clone()); // we clone it so that it can be used later when attaching to renderer
    renderer.pearls.add(mesh_renderer);

    // create model rotator
    let rotator = TransformRotator {
        transform: model_transform,
        rotation: 0.,
        speed: 2.,
    }
    .as_pearl();
    app.stages.add_pearl(rotator);

    // add the renderer and run the app
    app.resources.add(renderer);
    MilkTeaRunner::run(app).unwrap();
}
