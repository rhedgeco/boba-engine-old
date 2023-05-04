use boba::prelude::*;

struct FullscreenSwitcher;

impl Pearl for FullscreenSwitcher {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<KeyboardInput>();
    }
}

impl EventListener<KeyboardInput> for FullscreenSwitcher {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<KeyboardInput>) {
        // check if a key was pressed
        if !data.is_pressed() {
            return;
        }

        // get necessary window resources
        let Some(windows) = data.resources.get_mut::<Windows>() else { return };
        let Some(main_window) = windows.get_window("main") else { return };

        // if the space key is pressed, toggle in and out of fullscreen
        if data.event.keycode() == Some(KeyCode::Space) {
            main_window.set_fullscreen(!main_window.fullscreen());
        }
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(FullscreenSwitcher);

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
