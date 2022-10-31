use boba_core::BobaRunner;

pub struct MilkTeaRunner {}

impl BobaRunner for MilkTeaRunner {
    fn init() -> Self {
        Self {}
    }

    fn run(&mut self, mut app: boba_core::BobaApp) {
        loop {
            app.update();
        }
    }
}
