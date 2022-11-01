use crate::BobaApp;

pub trait BobaRunner {
    fn add_stages_and_resources(&mut self, app: &mut BobaApp);
    fn run(&mut self, app: BobaApp);
}
