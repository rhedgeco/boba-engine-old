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
        let Some(windows) = data.resources.get::<MilkTeaWindows>() else { return };
        let Some(main_window) = windows.get("main") else { return };

        // if the space key is pressed, toggle in and out of fullscreen
        // borderless fullscreen is used by default
        if data.keycode() == Some(KeyCode::Space) {
            match main_window.fullscreen() {
                Some(_) => main_window.set_fullscreen(None),
                None => main_window.set_fullscreen(Some(Fullscreen::Borderless(None))),
            }
        }

        // if the E key is pressed, move into exclusive fullscreen
        // this will sometimes mess up display monitor settings on linux
        if data.keycode() == Some(KeyCode::E) {
            let Some(monitor) = main_window.current_monitor() else { return };
            let Some(video_mode) = monitor.video_modes().next() else { return };
            println!("Using Video Mode: {video_mode}");
            main_window.set_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
        }
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(FullscreenSwitcher);

    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(LogicalSize::new(1280, 800));

    milk_tea.run(window, TaroBuilder::new()).unwrap();
}
