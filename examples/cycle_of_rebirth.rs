use boba::prelude::*;
use boba_core::{
    pearls::map::{EventWorldView, PearlMap, PearlMut},
    EventListener, EventRegistrar,
};

struct SelfRebirth {
    count: u32,
}

impl SelfRebirth {
    pub fn new(count: u32) -> Self {
        Self { count }
    }
}

impl Pearl for SelfRebirth {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for SelfRebirth {
    fn callback(pearl: PearlMut<Self>, mut world: EventWorldView<Update>) {
        println!("THE CYCLE OF REBIRTH CONTINUES! Count: {}", pearl.count);
        let child = SelfRebirth::new(pearl.count + 1);
        world.pearls.destroy(pearl.link());
        world.pearls.insert(child);
    }
}

fn main() {
    env_logger::init();
    let mut pearls = PearlMap::new();
    pearls.insert(SelfRebirth { count: 0 });
    MilkTeaHeadless::run(pearls, BobaResources::new());
}
