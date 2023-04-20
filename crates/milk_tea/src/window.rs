use boba_core::{pearls::map::BobaPearls, BobaResources};
use winit::window::Window;

pub trait WindowManager: 'static {
    fn window(&self) -> &Window;
    fn render(&mut self, pearls: &mut BobaPearls, resources: &mut BobaResources);
}

pub trait MilkTeaBuilder: 'static {
    type Window: WindowManager;
    fn build(self, window: Window) -> Self::Window;
}

pub struct MilkTeaWindow {
    manager: Box<dyn WindowManager>,
}

impl MilkTeaWindow {
    pub(super) fn new(manager: impl WindowManager) -> Self {
        Self {
            manager: Box::new(manager),
        }
    }

    pub(super) fn manager(&mut self) -> &mut Box<dyn WindowManager> {
        &mut self.manager
    }

    pub fn window(&self) -> &Window {
        self.manager.window()
    }
}
