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
    fn callback(pearl: &mut PearlData<Self>, mut event: EventData<Update>) {
        println!("FPS: {}", 1. / event.delta_time);
        for _ in 0..pearl.count {
            let link = event.pearls.queue_insert(DummyItem::default());
            event.pearls.queue_destroy(link);
        }
    }
}

fn main() {
    env_logger::init();
    let mut pearls = BobaPearls::new();
    pearls.insert(LimitTestPearl { count: 10000 });
    MilkTeaHeadless::run(pearls, BobaResources::new());
}
