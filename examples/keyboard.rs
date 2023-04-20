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
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(KeyboardPrinter);
    milk_tea.run(TaroBuilder::new()).unwrap();
}
