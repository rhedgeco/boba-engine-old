use boba::prelude::*;
use milk_tea::{
    winit::event::{ElementState, KeyboardInput, VirtualKeyCode},
    MilkTeaEvent,
};
use std::{f32::consts::PI, fs::File};
use taro_standard_shaders::{passes::UnlitRenderPass, UnlitShader, UnlitShaderInit};

pub struct Rotator {
    pub rotate: bool,
    pub transform: Pearl<BobaTransform>,
    pub current_rot: f32,
    pub speed: f32,
}

register_pearl_stages!(Rotator: BobaUpdate, MilkTeaEvent<KeyboardInput>);

impl PearlStage<MilkTeaEvent<KeyboardInput>> for Rotator {
    fn update(&mut self, data: &KeyboardInput, _: &mut BobaResources) -> BobaResult {
        let Some(key) = &data.virtual_keycode else {
            return Ok(());
        };

        if key != &VirtualKeyCode::Space {
            return Ok(());
        }

        match data.state {
            ElementState::Pressed => self.rotate = true,
            ElementState::Released => self.rotate = false,
        }

        Ok(())
    }
}

impl PearlStage<BobaUpdate> for Rotator {
    fn update(&mut self, delta: &f32, _resources: &mut BobaResources) -> BobaResult {
        if !self.rotate {
            return Ok(());
        }

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
    let mut camera = TaroCamera::new_simple(
        BobaTransform::from_position_look_at(Vec3::new(0., 1., 2.), Vec3::ZERO),
        TaroCameraSettings {
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        },
    );

    // add unlit render pass for testing
    camera.passes.append(UnlitRenderPass);

    // create texture for mesh
    let tex_view = Texture2DView::new(include_bytes!("boba-logo.png")).unwrap();

    // create a mesh to be rendered
    let renderer = TaroMeshRenderer::new_simple(
        BobaTransform::from_position(Vec3::ZERO),
        Mesh::new(File::open("cube.obj").unwrap()).unwrap(),
        TaroShader::<UnlitShader>::new(UnlitShaderInit::new(tex_view, Sampler::new())),
    );

    // create a rotator object that links to the renderers transform
    let rotator = Pearl::wrap(Rotator {
        rotate: false,
        transform: renderer.transform.clone(),
        current_rot: 0.,
        speed: 3.,
    });
    app.registry.add(&rotator);

    // create TaroCameras resource and add it
    let mut cameras = TaroCameras::default();
    cameras.cameras.push(camera);
    app.resources.add(cameras);

    // create TaroRenderPearls resource and add it
    let mut render_pearls = TaroRenderPearls::default();
    render_pearls.add(Pearl::wrap(renderer));
    app.resources.add(render_pearls);

    // run the app
    app.run().unwrap();
}
