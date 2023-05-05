use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<Update>) {
        println!("FPS: {}", 1. / data.event.delta);
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(UpdatePrinter);

    let transform = milk_tea
        .pearls
        .insert(Transform::new(TransformData::default()));

    milk_tea.pearls.insert(Taro3DCamera::new(transform));

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
