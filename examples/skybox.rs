use boba::prelude::*;
use taro_3d::pipelines::UnlitPipeline;

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();

    let cam_transform = milk_tea
        .pearls
        .insert(Transform::new(TransformData::default()));

    milk_tea.pearls.insert(Taro3DCamera::with_settings(
        cam_transform,
        TaroCameraSettings {
            pipeline: Box::new(UnlitPipeline),
            ..Default::default()
        },
    ));

    milk_tea.resources.insert(TaroSkybox::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
    });

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
