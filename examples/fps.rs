use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: PearlMut<Self>, world: EventWorldView<Update>) {
        println!("FPS: {}", 1. / world.event.delta_time);
    }
}

fn main() {
    env_logger::init();
    let mut pearls = PearlMap::new();
    pearls.insert(UpdatePrinter);

    let resources = BobaResources::new();
    let taro = TaroBuilder::new();
    MilkTeaWindow::new().run(pearls, resources, taro).unwrap();
}
