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
    fn callback(pearl: &mut PearlData<Self>, mut event: BobaEventData<Update>) {
        println!("FPS: {}", 1. / event.delta_time());
        for _ in 0..pearl.count {
            let link = event.pearls.queue_insert(DummyItem::default());
            event.pearls.queue_destroy(link);
        }
    }
}

fn main() {
    let mut milk_tea = MilkTeaHeadless::new();
    milk_tea.pearls.insert(LimitTestPearl { count: 10000 });
    milk_tea.run();
}
