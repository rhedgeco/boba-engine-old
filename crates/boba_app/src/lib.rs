mod milk_tea;

pub use milk_tea::*;

pub enum BobaEvent {
    Update,
}

pub trait BobaApp {
    fn handle_event(&mut self, event: BobaEvent);
}

pub trait BobaRunner {
    fn run<A: 'static + BobaApp>(self, app: A);
}
