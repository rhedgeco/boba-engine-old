use crate::BobaApp;

pub trait BobaRunner {
    fn run(&mut self, app: BobaApp);
}
