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
    let mut app = Bobarista::default();

    // create textures
    let boba_texture =
        Texture2DView::new(include_bytes!("../readme_assets/boba-logo.png")).unwrap();
    let grid_texture = Texture2DView::new(include_bytes!("../assets/uv_grid.png")).unwrap();

    // create shaders
    let boba_shader =
        Shader::<UnlitShader>::new(UnlitShaderInit::new(boba_texture, Sampler::new()));
    let grid_shader =
        Shader::<UnlitShader>::new(UnlitShaderInit::new(grid_texture, Sampler::new()));

    // create mesh renderers
    let plane_renderer = TaroMeshRenderer::new_simple(
        BobaTransform::from_position(Vec3::ZERO),
        Mesh::new(File::open("./assets/plane.obj").unwrap()).unwrap(),
        grid_shader.clone(),
    );

    let sphere_renderer = TaroMeshRenderer::new_simple(
        BobaTransform::from_position(Vec3::Y * 0.5),
        Mesh::new(File::open("./assets/sphere.obj").unwrap()).unwrap(),
        boba_shader.clone(),
    );

    let mut suzanne_renderer = TaroMeshRenderer::new_simple(
        BobaTransform::from_position_scale(Vec3::X * 1.5, Vec3::ONE * 0.5),
        Mesh::new(File::open("./assets/suzanne.obj").unwrap()).unwrap(),
        grid_shader.clone(),
    );

    let mut cube_renderer = TaroMeshRenderer::new_simple(
        BobaTransform::from_position_scale(-Vec3::X * 1.5, Vec3::ONE * 0.5),
        Mesh::new(File::open("./assets/cube.obj").unwrap()).unwrap(),
        boba_shader.clone(),
    );

    // set parents
    suzanne_renderer
        .transform
        .set_parent(sphere_renderer.transform.clone())
        .unwrap();
    cube_renderer
        .transform
        .set_parent(sphere_renderer.transform.clone())
        .unwrap();

    // create a rotator object that links to the renderers transform
    let rotator = Pearl::wrap(Rotator::new(sphere_renderer.transform.clone(), 3.));
    app.registry.add(rotator);

    // create TaroRenderPearls resource and add it
    let mut render_pearls = TaroRenderPearls::default();
    render_pearls.add(Pearl::wrap(plane_renderer));
    render_pearls.add(Pearl::wrap(sphere_renderer));
    render_pearls.add(Pearl::wrap(suzanne_renderer));
    render_pearls.add(Pearl::wrap(cube_renderer));
    app.resources.add(render_pearls);

    // create camera with transform
    let mut camera = TaroCamera::new_simple(
        BobaTransform::from_position_look_at(Vec3::new(0., 2., 3.), Vec3::Y * 0.5),
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
    app.run::<TaroGraphicsAdapter>().unwrap();
}
