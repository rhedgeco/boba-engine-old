use boba::prelude::*;

struct SelfDestroy;

impl Pearl for SelfDestroy {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for SelfDestroy {
    fn callback(pearl: PearlLink<Self>, _: &Update, view: EventView<Self>) {
        println!("GOODBYE CRUEL WORLD!");
        view.commands.destroy_pearl(&pearl);
    }
}

fn main() {
    env_logger::init();
    let mut world = BobaWorld::new();
    world.insert_pearl(SelfDestroy);
    MilkTeaHeadless::run(world);
}
