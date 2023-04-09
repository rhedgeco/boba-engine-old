use boba::prelude::*;

struct SelfDestroy;

impl Pearl for SelfDestroy {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for SelfDestroy {
    fn callback(&mut self, _: &Update, view: EventView<Self>) {
        println!("GOODBYE CRUEL WORLD!");
        view.commands.destroy_pearl(view.pearls.current());
    }
}

fn main() {
    env_logger::init();
    let mut world = BobaWorld::new();
    world.insert_pearl(SelfDestroy);
    MilkTeaHeadless::run(world);
}
