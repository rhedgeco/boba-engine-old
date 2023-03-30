use boba_hybrid::{events::EventListener, handle_map::Handle, register_pearl, BobaApp, World};
use milk_tea_manager::{events::MilkTeaEvent, winit::event::ElementState, MilkTea};
use taro_renderer::TaroRenderer;

struct InputPrinter;

register_pearl!(InputPrinter => MilkTeaEvent);

impl EventListener<MilkTeaEvent> for InputPrinter {
    fn callback(_: &Handle<Self>, event: &MilkTeaEvent, _: &mut World) {
        match event {
            MilkTeaEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } if input.state == ElementState::Pressed => {
                println!("Keyboard Input: {:?}", input.virtual_keycode);
            }
            MilkTeaEvent::MouseInput {
                device_id: _,
                state,
                button,
            } if state == &ElementState::Pressed => {
                println!("Mouse Button: {:?}", button);
            }
            MilkTeaEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
            } => {
                println!("Mouse Wheel: {:?}", delta);
            }
            _ => (),
        }
    }
}

fn main() {
    let mut app = BobaApp::new();
    app.insert_pearl(InputPrinter);

    let manager = MilkTea::<TaroRenderer>::new();
    app.run(manager).unwrap();
}
