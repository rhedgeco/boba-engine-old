use boba::prelude::*;

#[derive(Default)]
struct DummyItem {
    _item: u128,
}

impl Pearl for DummyItem {}

struct LimitTestPearl {
    count: u32,
}

impl Pearl for LimitTestPearl {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for LimitTestPearl {
    fn callback(pearl: PearlMut<Self>, mut world: EventWorldView<Update>) {
        println!("FPS: {}", 1. / world.event.delta_time);
        for _ in 0..pearl.count {
            let link = world.pearls.queue_insert(DummyItem::default());
            world.pearls.queue_destroy(link);
        }
    }
}

fn main() {
    env_logger::init();
    let mut pearls = PearlMap::new();
    pearls.insert(LimitTestPearl { count: 10000 });
    MilkTeaHeadless::run(pearls, BobaResources::new());
}
