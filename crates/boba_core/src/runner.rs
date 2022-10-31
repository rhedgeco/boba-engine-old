use crate::BobaApp;

pub trait BobaRunner {
    fn init() -> Self;
    fn run(&mut self, app: BobaApp);
}
