use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: &mut PearlData<Self>, event: BobaEventData<Update>) {
        println!("FPS: {}", 1. / event.delta_time());
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(UpdatePrinter);

    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(LogicalSize::new(1280, 800));

    milk_tea.run(window, TaroBuilder::new()).unwrap();
}
