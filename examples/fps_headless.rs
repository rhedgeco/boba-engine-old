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
    let mut pearls = BobaPearls::new();
    pearls.insert(UpdatePrinter);
    MilkTeaHeadless::run(pearls, BobaResources::new());
}
