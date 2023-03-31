use boba_core::{
    events::{EventListener, EventRegistrar},
    handle_map::Handle,
    pearls::Pearl,
    BobaApp, World,
};
use milk_tea::{events::Update, MilkTeaWindow};
use taro_renderer::TaroBuilder;

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
    let mut app = BobaApp::new();
    app.insert_pearl(UpdatePrinter);
    MilkTeaWindow::new().run(app, TaroBuilder::new()).unwrap();
}
