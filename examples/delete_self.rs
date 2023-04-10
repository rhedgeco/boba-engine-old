use boba::prelude::*;

struct SelfDestroy;

impl Pearl for SelfDestroy {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for SelfDestroy {
    fn callback(pearl: PearlLink<Self>, event: EventData<Update>) {
        println!("GOODBYE CRUEL WORLD!");
        event.commands.destroy_pearl(pearl.link());
    }
}

fn main() {
    env_logger::init();
    let mut world = BobaWorld::new();
    world.pearls.insert(SelfDestroy);
    MilkTeaHeadless::run(world);
}
