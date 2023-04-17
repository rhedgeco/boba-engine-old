use boba::prelude::*;

struct RebirthPearl {
    count: u32,
}

impl Pearl for RebirthPearl {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for RebirthPearl {
    fn callback(pearl: &mut PearlData<Self>, mut world: EventWorldView<Update>) {
        println!("GOODBYE CRUEL WORLD!");
        world.pearls.queue_destroy(pearl.handle());

        let count = pearl.count + 1;
        println!("THE CYCLE OF REBIRTH SHALL CONTINUE! Count: {count}");
        let child = RebirthPearl { count };
        world.pearls.queue_insert(child);
    }
}

fn main() {
    env_logger::init();
    let mut pearls = BobaPearls::new();
    pearls.insert(RebirthPearl { count: 0 });
    MilkTeaHeadless::run(pearls, BobaResources::new());
}
