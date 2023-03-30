use boba_hybrid::{events::EventListener, handle_map::Handle, register_pearl, BobaApp, World};
use milk_tea_manager::{events::MilkTeaUpdate, MilkTea};
use taro_renderer::TaroRenderer;

struct UpdatePrinter;

register_pearl!(UpdatePrinter => MilkTeaUpdate);

impl EventListener<MilkTeaUpdate> for UpdatePrinter {
    fn callback(_: &Handle<Self>, update: &MilkTeaUpdate, _: &mut World) {
        println!("FPS: {}", 1. / update.delta_time);
    }
}

fn main() {
    let mut app = BobaApp::new();
    app.insert_pearl(UpdatePrinter);

    let manager = MilkTea::<TaroRenderer>::new();
    app.run(manager).unwrap();
}
