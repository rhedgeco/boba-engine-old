use boba_hybrid::{events::EventListener, pearls::Pearl, BobaApp, World};
use handle_map::Handle;
use milk_tea_manager::{events::MilkTeaUpdate, MilkTea};

struct UpdatePrinter;

impl EventListener<MilkTeaUpdate> for UpdatePrinter {
    fn callback(_: &Handle<Self>, _: &MilkTeaUpdate, _: &mut World) {
        println!("UPDATE!");
    }
}

impl Pearl for UpdatePrinter {
    fn register(registrar: &mut impl boba_hybrid::events::EventRegistrar<Self>) {
        registrar.listen_for::<MilkTeaUpdate>();
    }
}

fn main() {
    let mut app = BobaApp::new();
    app.insert_pearl(UpdatePrinter);

    let manager = MilkTea::new("Milk Tea Window Test", (640, 480));
    app.run(manager).unwrap();
}
