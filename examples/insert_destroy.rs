use boba::prelude::*;

struct RebirthPearl {
    count: u32,
}

impl Pearl for RebirthPearl {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }

    fn on_insert(handle: Handle<Self>, pearls: &mut impl PearlProvider) {
        // while it is guaranteed that the pearl will be valid
        // it is good practice to still not use `unwrap` when possible
        let Some(pearl) = pearls.get(handle) else { return };
        println!("THE CYCLE OF REBIRTH CONTINUES! Count: {}", pearl.count);
    }

    fn on_remove(_: &mut PearlData<Self>, _: &mut impl PearlProvider) {
        // print goodbye on removal from a pearl map
        println!("GOODBYE CRUEL WORLD!");
    }
}

impl EventListener<Update> for RebirthPearl {
    fn callback(pearl: &mut PearlData<Self>, mut world: EventData<Update>) {
        // queue destroy self
        world.pearls.queue_destroy(pearl.handle());

        // queue insert of new child with increased count
        world.pearls.queue_insert(RebirthPearl {
            count: pearl.count + 1,
        });
    }
}

fn main() {
    let mut milk_tea = MilkTeaHeadless::new();
    milk_tea.pearls.insert(RebirthPearl { count: 0 });
    milk_tea.run();
}
