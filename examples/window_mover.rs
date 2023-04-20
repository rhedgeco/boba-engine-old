use boba::prelude::*;
use milk_tea::MilkTeaWindow;

struct WindowMover {
    move_speed: f64,
    move_buffer: f64,
}

impl WindowMover {
    pub fn new(move_speed: f64) -> Self {
        Self {
            move_speed,
            move_buffer: 0.,
        }
    }
}

impl Pearl for WindowMover {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for WindowMover {
    fn callback(pearl: &mut PearlData<Self>, event: EventData<Update>) {
        let Some(window) = event.resources.get::<MilkTeaWindow>() else { return };
        let Ok(mut position) = window.window().outer_position() else { return };

        pearl.move_buffer += pearl.move_speed * event.delta_time();
        position.x += pearl.move_buffer.floor() as i32;
        pearl.move_buffer %= 1.;

        window.window().set_outer_position(position);
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(WindowMover::new(1000.));
    milk_tea.run(TaroBuilder::new()).unwrap();
}
