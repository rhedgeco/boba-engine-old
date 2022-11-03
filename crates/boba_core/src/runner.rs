use crate::BobaApp;

#[allow(unused_variables)]
pub trait BobaRunner {
    fn run(&mut self, app: BobaApp);
    fn setup(&mut self, app: &mut BobaApp) {}
}
