pub trait BobaRunner {
    fn run(&mut self, app: &mut BobaApp);
}

pub struct BobaApp {}

impl BobaApp {
    pub fn new() -> Self {
        Self {}
    }
}
