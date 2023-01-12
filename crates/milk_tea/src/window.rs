use boba_core::{BobaResources, BobaResult, PearlRegistry};
use log::error;
use winit::{dpi::PhysicalSize, window::Window};

pub trait MilkTeaRenderAdapter: 'static {
    fn build(window: &Window) -> Self
    where
        Self: Sized;
    fn render(
        &mut self,
        window_size: PhysicalSize<u32>,
        registry: &mut PearlRegistry,
        resources: &mut BobaResources,
    ) -> BobaResult;
}

pub struct MilkTeaWindow<T: MilkTeaRenderAdapter> {
    renderer: T,
    window: Window,
}

impl<T: MilkTeaRenderAdapter> MilkTeaWindow<T> {
    pub(crate) fn new(window: Window) -> Self {
        Self {
            renderer: T::build(&window),
            window,
        }
    }

    pub(crate) fn render(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources) {
        let size = self.window.inner_size();
        if let Err(e) = self.renderer.render(size, registry, resources) {
            error!("There was an error when rendering the milk tea window. Error: {e}");
        }
    }
}
