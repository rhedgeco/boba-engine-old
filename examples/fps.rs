use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: &mut PearlData<Self>, event: EventData<Update>) {
        println!("FPS: {}", 1. / event.delta_time());
    }
}

fn main() {
    env_logger::init();
    let mut pearls = BobaPearls::new();
    pearls.insert(UpdatePrinter);

    let resources = BobaResources::new();
    let taro = TaroBuilder::new();
    MilkTeaWindow::new().run(pearls, resources, taro).unwrap();
}
