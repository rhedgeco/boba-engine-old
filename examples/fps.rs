use boba_core::{
    events::{EventData, EventListener},
    handle_map::Handle,
    register_pearl, BobaApp,
};
use milk_tea::{events::Update, MilkTeaWindow};
use taro_renderer::TaroBuilder;

struct UpdatePrinter;

register_pearl!(UpdatePrinter => Update);

impl EventListener<Update> for UpdatePrinter {
    fn callback(_: &Handle<Self>, event: EventData<Update>) {
        println!("FPS: {}", 1. / event.data.delta_time);
    }
}

fn main() {
    let mut app = BobaApp::new();
    app.insert_pearl(UpdatePrinter);
    MilkTeaWindow::new().run(app, TaroBuilder::new()).unwrap();
}
