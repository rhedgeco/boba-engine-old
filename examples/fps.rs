use boba::prelude::*;

struct UpdatePrinter;

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: &Handle<Self>, update: &Update, _: &mut World) {
        println!("FPS: {}", 1. / update.delta_time);
    }
}

fn main() {
    env_logger::init();
    let mut app = BobaApp::new();
    app.insert_pearl(UpdatePrinter);
    MilkTeaWindow::new().run(app, TaroBuilder::new()).unwrap();
}
