use boba::prelude::*;

struct KeyboardPrinter;

impl Pearl for KeyboardPrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<KeyboardInput>();
        registrar.listen_for::<MouseMotion>();
    }
}

impl EventListener<MouseMotion> for KeyboardPrinter {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<MouseMotion>) {
        println!("Mouse Motion: ({}, {})", data.delta_x, data.delta_y);
    }
}

impl EventListener<KeyboardInput> for KeyboardPrinter {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<KeyboardInput>) {
        if !data.is_pressed() {
            return;
        }

        println!("Key Pressed: {:?}", data.keycode());
        if data.keycode() == Some(KeyCode::Escape) {
            if let Some(commands) = data.resources.get_mut::<Commands>() {
                commands.exit_app();
            }
        }
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(KeyboardPrinter);

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
