use boba_hybrid::{events::EventListener, register_pearl, BobaApp, World};
use handle_map::Handle;
use milk_tea_manager::{events::MilkTeaUpdate, MilkTea};

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

    let manager = MilkTea::new("Milk Tea Window Test", (640, 480));
    app.run(manager).unwrap();
}
