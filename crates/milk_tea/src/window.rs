use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::window::Window;

pub trait MilkTeaWindow: 'static {
    fn window(&self) -> &Window;
    fn render(&mut self, pearls: &mut BobaPearls, resources: &mut BobaResources);
}

pub trait MilkTeaBuilder: 'static {
    type Window: MilkTeaWindow;
    fn build(self, window: Window) -> Self::Window;
}
