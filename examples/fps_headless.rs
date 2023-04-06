use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(&mut self, event: &Update, _: &mut WorldView) {
        println!("FPS: {}", 1. / event.delta_time);
    }
}

fn main() {
    env_logger::init();
    let mut world = BobaWorld::new();
    world.insert_pearl(UpdatePrinter);
    MilkTeaHeadless::run(world);
}
