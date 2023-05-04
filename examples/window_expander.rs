use boba::prelude::*;

struct WindowExpander {
    pub amount: i32,
}

impl Pearl for WindowExpander {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<KeyboardInput>();
    }
}

impl EventListener<KeyboardInput> for WindowExpander {
    fn callback(pearl: &mut PearlData<Self>, data: BobaEventData<KeyboardInput>) {
        if !data.event.is_pressed() {
            return;
        }

        let Some(windows) = data.resources.get_mut::<Windows>() else { return };
        let Some(main_window) = windows.get_window("main") else { return };

        if data.event.keycode() == Some(KeyCode::Right) {
            main_window.expand(pearl.amount as i32, 0)
        } else if data.event.keycode() == Some(KeyCode::Left) {
            main_window.expand(-pearl.amount as i32, 0)
        } else if data.event.keycode() == Some(KeyCode::Down) {
            main_window.expand(0, pearl.amount as i32)
        } else if data.event.keycode() == Some(KeyCode::Up) {
            main_window.expand(0, -pearl.amount as i32)
        }
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(WindowExpander { amount: 10 });

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
