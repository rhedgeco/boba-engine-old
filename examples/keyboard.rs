use boba::prelude::*;

struct KeyboardPrinter;

impl Pearl for KeyboardPrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<KeyboardInput>();
    }
}

impl EventListener<KeyboardInput> for KeyboardPrinter {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<KeyboardInput>) {
        if !data.is_pressed() {
            return;
        }

        println!("Key Pressed: {:?}", data.keycode());
        if data.keycode() == Some(KeyCode::Escape) {
            if let Some(commands) = data.resources.get_mut::<MilkTeaCommands>() {
                commands.exit_app();
            }
        }
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(KeyboardPrinter);

    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(LogicalSize::new(1280, 800));

    milk_tea.run(window, TaroBuilder::new()).unwrap();
}
