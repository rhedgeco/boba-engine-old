use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: PearlLink<Self>, event: EventData<Update>) {
        println!("FPS: {}", 1. / event.data.delta_time);
    }
}

fn main() {
    env_logger::init();
    let mut world = BobaWorld::new();
    world.pearls.insert(UpdatePrinter);
    MilkTeaWindow::new().run(world, TaroBuilder::new()).unwrap();
}
