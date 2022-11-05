use crate::BobaApp;

pub trait BobaPlugin {
    fn setup(self, app: &mut BobaApp);
}
