use boba::prelude::*;
use milk_tea::{
    events::MilkTeaEvent,
    winit::event::{ElementState, KeyboardInput, VirtualKeyCode},
};
use std::{f32::consts::PI, fs::File};
use taro_standard_shaders::{passes::UnlitRenderPass, UnlitShader, UnlitShaderInit};

pub struct Rotator {
    current_rot: f32,
    rotate_direction: f32,

    pub transform: Pearl<BobaTransform>,
    pub speed: f32,
}

impl Rotator {
    pub fn new(transform: Pearl<BobaTransform>, speed: f32) -> Self {
        Self {
            current_rot: 0.,
            rotate_direction: 0.,
            transform,
            speed,
        }
    }
}

register_pearl_stages!(Rotator: BobaUpdate, MilkTeaEvent<KeyboardInput>);

impl PearlStage<MilkTeaEvent<KeyboardInput>> for Rotator {
    fn update(pearl: &Pearl<Self>, data: &KeyboardInput, _: &mut BobaResources) -> BobaResult {
        let mut pearl = pearl.borrow_mut()?;

        let rotate_direction = match &data.virtual_keycode {
            Some(VirtualKeyCode::Right) => 1.,
            Some(VirtualKeyCode::Left) => -1.,
            _ => 0.,
        };

        match data.state {
            ElementState::Pressed => pearl.rotate_direction = rotate_direction,
            ElementState::Released => pearl.rotate_direction = 0.,
        }

        Ok(())
    }
}

impl PearlStage<BobaUpdate> for Rotator {
    fn update(pearl: &Pearl<Self>, delta: &f32, _resources: &mut BobaResources) -> BobaResult {
        let mut pearl = pearl.borrow_mut()?;

        pearl.current_rot += pearl.speed * pearl.rotate_direction * delta;
        pearl.current_rot %= 2. * PI;

        let mut transform = pearl.transform.borrow_mut()?;
        transform.set_local_rotation(Quat::from_axis_angle(Vec3::Y, pearl.current_rot));

        println!("FPS: {}", 1. / delta);
        Ok(())
    }
}

fn main() {
    // create app
    let mut app = Bobarista::<TaroMilkTea>::default();

    // create meshes
    let cube = Mesh::new(File::open("./assets/cube.obj").unwrap()).unwrap();
    let sphere = Mesh::new(File::open("./assets/sphere.obj").unwrap()).unwrap();

    // create texture for mesh
    let tex_view = Texture2DView::new(include_bytes!("../readme_assets/boba-logo.png")).unwrap();

    // create shader for the mesh
    let shader = Shader::<UnlitShader>::new(UnlitShaderInit::new(tex_view, Sampler::new()));

    // create a mesh to be rendered
    let renderer = TaroMeshRenderer::new_simple(
        BobaTransform::from_position(Vec3::ZERO),
        sphere.clone(),
        shader.clone(),
    );

    // create another mesh to be rendered
    let mut renderer2 = TaroMeshRenderer::new_simple(
        BobaTransform::from_position_scale(Vec3::X * 1.5, Vec3::ONE * 0.5),
        cube.clone(),
        shader.clone(),
    );

    // create another mesh to be rendered
    let mut renderer3 = TaroMeshRenderer::new_simple(
        BobaTransform::from_position_scale(-Vec3::X * 1.5, Vec3::ONE * 0.5),
        cube.clone(),
        shader.clone(),
    );

    // set parents
    renderer2
        .transform
        .set_parent(renderer.transform.clone())
        .unwrap();
    renderer3
        .transform
        .set_parent(renderer.transform.clone())
        .unwrap();

    // create a rotator object that links to the renderers transform
    let rotator = Pearl::wrap(Rotator::new(renderer.transform.clone(), 3.));
    app.registry.add(&rotator);

    // create TaroRenderPearls resource and add it
    let mut render_pearls = TaroRenderPearls::default();
    render_pearls.add(Pearl::wrap(renderer));
    render_pearls.add(Pearl::wrap(renderer2));
    render_pearls.add(Pearl::wrap(renderer3));
    app.resources.add(render_pearls);

    // create camera with transform
    let mut camera = TaroCamera::new_simple(
        BobaTransform::from_position_look_at(Vec3::new(0., 1., 2.), Vec3::ZERO),
        TaroCameraSettings {
            fovy: 60.0,
            znear: 0.1,
            zfar: 100.0,
        },
    );

    // add unlit render pass for testing
    camera.passes.append(UnlitRenderPass);

    // create TaroCameras resource and add it
    let mut cameras = TaroCameras::default();
    cameras.cameras.push(camera);
    app.resources.add(cameras);

    // run the app
    app.run().unwrap();
}
