use boba::prelude::*;

struct KeyboardPrinter;

impl Pearl for KeyboardPrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<KeyboardInput>();
    }
}

impl EventListener<KeyboardInput> for KeyboardPrinter {
    fn callback(_: &mut PearlData<Self>, event: EventData<KeyboardInput>) {
        if !event.is_pressed() {
            return;
        }

        println!("Key Pressed: {:?}", event.keycode());
        if event.keycode() == Some(KeyCode::Escape) {
            if let Some(commands) = event.resources.get_mut::<MilkTeaCommands>() {
                commands.exit_app();
            }
        }
    }
}

fn main() {
    let mut pearls = BobaPearls::new();
    pearls.insert(KeyboardPrinter);

    let resources = BobaResources::new();
    let taro = TaroBuilder::new();
    MilkTeaWindow::new().run(pearls, resources, taro).unwrap();
}
