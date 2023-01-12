use boba::prelude::*;
use boba_rapier3d::{
    rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder},
    stages::OnRapierUpdate,
    RapierPhysics,
};
use std::fs::File;
use taro_standard_shaders::{passes::UnlitRenderPass, UnlitShader, UnlitShaderInit};

fn main() {
    // create app
    let mut app = Bobarista::<TaroGraphicsAdapter>::default();

    // create physics handler and rigidbody transforms
    let mut physics = RapierPhysics::new();
    let ground_transform = physics.create_transform(
        RigidBodyBuilder::fixed().build(),
        ColliderBuilder::cuboid(5., 0.01, 5.).build(),
    );
    let ball_transform = physics.create_transform(
        RigidBodyBuilder::dynamic()
            .translation(Vec3::new(-0.18, 1.5, 0.).into())
            .build(),
        ColliderBuilder::ball(0.5).build(),
    );
    let cube_transform = physics.create_transform(
        RigidBodyBuilder::dynamic()
            .translation(Vec3::new(0.18, 3., -0.15).into())
            .build(),
        ColliderBuilder::cuboid(0.5, 0.5, 0.5).build(),
    );

    // create shaders and mesh renderers
    let boba_shader = Shader::<UnlitShader>::new(UnlitShaderInit::new(
        Texture2DView::new(include_bytes!("../readme_assets/boba-logo.png")).unwrap(),
        Sampler::new(),
    ));

    let grid_shader = Shader::<UnlitShader>::new(UnlitShaderInit::new(
        Texture2DView::new(include_bytes!("../assets/uv_grid.png")).unwrap(),
        Sampler::new(),
    ));

    let plane_renderer = TaroMeshRenderer::new(
        ground_transform.clone(),
        Mesh::new(File::open("./assets/plane.obj").unwrap()).unwrap(),
        grid_shader.clone(),
    );

    let sphere_renderer = TaroMeshRenderer::new(
        ball_transform.clone(),
        Mesh::new(File::open("./assets/sphere.obj").unwrap()).unwrap(),
        boba_shader.clone(),
    );

    let cube_renderer = TaroMeshRenderer::new(
        cube_transform.clone(),
        Mesh::new(File::open("./assets/cube.obj").unwrap()).unwrap(),
        boba_shader.clone(),
    );

    // create TaroRenderPearls to hold mesh renderers
    let mut render_pearls = TaroRenderPearls::default();
    render_pearls.add(Pearl::wrap(plane_renderer));
    render_pearls.add(Pearl::wrap(sphere_renderer));
    render_pearls.add(Pearl::wrap(cube_renderer));

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

    // create TaroCameras resource
    let mut cameras = TaroCameras::default();
    cameras.cameras.push(camera);

    // add all required stages
    app.main_stages.append(OnRapierUpdate::default());

    // add all created resources
    app.resources.add(physics);
    app.resources.add(render_pearls);
    app.resources.add(cameras);

    // run the app
    app.run().unwrap();
}
