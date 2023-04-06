use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(&mut self, event: &Update, _: &mut boba_core::WorldView) {
        println!("FPS: {}", 1. / event.delta_time);
    }
}

fn main() {
    env_logger::init();
    let mut app = BobaWorld::new();
    app.insert_pearl(UpdatePrinter);
    MilkTeaWindow::new().run(app, TaroBuilder::new()).unwrap();
}
