use boba::prelude::*;
use boba_rapier3d::{
    rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder},
    stages::OnRapierUpdate,
    RapierPhysics,
};
use std::fs::File;
use taro_core::{
    data::{
        texture::{Texture2D, Texture2DView},
        Mesh, PointLight,
    },
    rendering::{shaders::LitShader, TaroMeshRenderer, TaroRenderPearls},
    wgpu::Color,
    TaroCamera,
};
use taro_deferred_pipeline::DeferredPipeline;
use taro_milk_tea::TaroGraphicsAdapter;

fn main() {
    // create app
    let mut app = MilkTeaApp::default();

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

    let boba_texture =
        Texture2D::from_bytes(include_bytes!("../readme_assets/boba-logo.png")).unwrap();
    let grid_texture = Texture2D::from_bytes(include_bytes!("../assets/uv_grid.png")).unwrap();

    let boba_shader = LitShader::new(
        Color::WHITE.into(),
        Texture2DView::from_texture(boba_texture),
    );
    let grid_shader = LitShader::new(
        Color::WHITE.into(),
        Texture2DView::from_texture(grid_texture),
    );

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

    // create a point light
    let point_light = PointLight::new_simple(Vec3::new(0., 0.1, 0.), Color::WHITE);

    // create TaroRenderPearls to hold mesh renderers
    let mut render_pearls = TaroRenderPearls::default();
    render_pearls.add(Pearl::wrap(plane_renderer));
    render_pearls.add(Pearl::wrap(sphere_renderer));
    render_pearls.add(Pearl::wrap(cube_renderer));
    render_pearls.add(Pearl::wrap(point_light));

    // create camera with transform
    let camera = TaroCamera::new_simple(
        BobaTransform::from_position_look_at(Vec3::new(0., 2., 3.), Vec3::Y * 0.5),
        DeferredPipeline,
    );

    // add all required stages
    app.main_stages.append(OnRapierUpdate::default());

    // add all created resources
    app.resources.add(physics);
    app.resources.add(render_pearls);
    app.resources.add(camera);

    // run the app
    app.run::<TaroGraphicsAdapter>().unwrap();
}
