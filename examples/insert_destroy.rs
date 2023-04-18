use boba::prelude::*;

struct RebirthPearl {
    count: u32,
}

impl Pearl for RebirthPearl {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }

    fn on_insert(&mut self, _: Handle<Self>) {
        // print rebirth count when inserted into a pearl map
        println!("THE CYCLE OF REBIRTH CONTINUES! Count: {}", self.count);
    }

    fn on_remove(&mut self) {
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
    let mut pearls = BobaPearls::new();
    pearls.insert(RebirthPearl { count: 0 });
    MilkTeaHeadless::run(pearls, BobaResources::new());
}
