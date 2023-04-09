use boba::prelude::*;

struct SelfDestroy {
    link: Option<Link<SelfDestroy>>,
}
impl Pearl for SelfDestroy {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for SelfDestroy {
    fn callback(&mut self, _: &Update, world: EventView) {
        println!("GOODBYE CRUEL WORLD!");
        if let Some(link) = self.link {
            world.commands.destroy_pearl(link);
        }
    }
}

fn main() {
    env_logger::init();
    let mut world = BobaWorld::new();
    let link = world.insert_pearl(SelfDestroy { link: None });
    world.get_pearl_mut(&link).unwrap().link = Some(link);
    MilkTeaHeadless::run(world);
}
