use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<Update>) {
        println!("FPS: {}", 1. / data.delta);
    }
}

fn main() {
    let mut milk_tea = MilkTeaHeadless::new();
    milk_tea.pearls.insert(UpdatePrinter);
    milk_tea.run();
}
