use boba::prelude::*;

struct WindowMover {
    move_speed: (f64, f64),
    move_collector: (f64, f64),
}

impl WindowMover {
    pub fn new(move_speed: (f64, f64)) -> Self {
        Self {
            move_speed,
            move_collector: (0., 0.),
        }
    }
}

impl Pearl for WindowMover {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for WindowMover {
    fn callback(pearl: &mut PearlData<Self>, data: BobaEventData<Update>) {
        pearl.move_collector.0 += pearl.move_speed.0 * data.event.delta;
        pearl.move_collector.1 += pearl.move_speed.1 * data.event.delta;

        let pixel_move_x = pearl.move_collector.0.trunc() as u32;
        let pixel_move_y = pearl.move_collector.1.trunc() as u32;
        pearl.move_collector.0 = pearl.move_collector.0.fract();
        pearl.move_collector.1 = pearl.move_collector.1.fract();

        let Some(windows) = data.resources.get_mut::<Windows>() else { return };
        let Some(main_window) = windows.get_window("main") else { return };
        main_window.move_position((pixel_move_x, pixel_move_y));
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(WindowMover::new((100., 100.)));

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
