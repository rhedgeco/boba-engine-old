use boba::prelude::*;
use std::f32::consts::PI;

use taro_renderer::{
    data_types::{TaroMesh, Vertex},
    pearls::TaroMeshRenderer,
    shading::TaroShader,
    TaroCamera, TaroCameraSettings, TaroCameras, TaroRenderPearls,
};
use taro_standard_shaders::{passes::UnlitRenderPass, UnlitShader};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.], normal: [1., 1., 1.], uv: [0., 1.] },
    Vertex { position: [0.5, -0.5, 0.], normal: [1., 0., 0.], uv: [1., 1.] },
    Vertex { position: [0.5, 0.5, 0.], normal: [0., 1., 0.], uv: [1., 0.] },
    Vertex { position: [-0.5, 0.5, 0.], normal: [0., 0., 1.], uv: [0., 0.] },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];

pub struct Rotator {
    pub transform: Pearl<BobaTransform>,
    pub current_rot: f32,
    pub speed: f32,
}

impl RegisterStages for Rotator {
    fn register(pearl: &Pearl<Self>, stages: &mut impl StageRegistrar) {
        stages.add(pearl.clone());
    }
}

impl PearlStage<BobaUpdate> for Rotator {
    fn update(&mut self, delta: &f32, _resources: &mut BobaResources) -> BobaResult {
        let mut transform = self.transform.borrow_mut()?;

        self.current_rot += self.speed * delta;
        self.current_rot %= 2. * PI;

        transform.set_local_rotation(Quat::from_axis_angle(Vec3::Y, self.current_rot));
        Ok(())
    }
}

fn main() {
    // create app
    let mut app = Bobarista::<TaroMilkTea>::default();

    // create camera with transform
    let mut camera_transform = BobaTransform::from_position(Vec3::new(0., 1., 2.));
    camera_transform.look_at(Vec3::ZERO);
    let camera_transform = Pearl::wrap(camera_transform);
    let mut camera = TaroCamera::new(
        TaroCameraSettings {
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        },
        camera_transform,
    );
    camera.passes.append(UnlitRenderPass);

    // create a mesh to be rendered
    let model_transform = Pearl::wrap(BobaTransform::from_position(Vec3::ZERO));
    let shader = TaroShader::<UnlitShader>::new();
    let mesh = TaroMesh::new(VERTICES, INDICES);
    let renderer = Pearl::wrap(TaroMeshRenderer::new(model_transform.clone(), mesh, shader));
    let rotator = Pearl::wrap(Rotator {
        transform: model_transform,
        current_rot: 0.,
        speed: 1.,
    });
    app.registry.add(&rotator);

    // create TaroCameras resource and add it
    let mut cameras = TaroCameras::default();
    cameras.cameras.push(camera);
    app.resources.add(cameras);

    // create TaroRenderPearls resource and add it
    let mut render_pearls = TaroRenderPearls::default();
    render_pearls.add(renderer);
    app.resources.add(render_pearls);

    // run the app
    app.run().unwrap();
}