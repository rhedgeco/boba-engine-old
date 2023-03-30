use boba_hybrid::{events::EventListener, handle_map::Handle, register_pearl, BobaApp, World};
use milk_tea::{events::Update, MilkTeaWindow};
use taro_renderer::TaroBuilder;

struct UpdatePrinter;

register_pearl!(UpdatePrinter => Update);

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
