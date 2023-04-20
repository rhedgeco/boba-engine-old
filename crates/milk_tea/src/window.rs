use boba_core::{pearls::map::BobaPearls, BobaResources};
use indexmap::IndexMap;
use log::error;
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

pub trait MilkTeaRenderer: 'static {
    fn window_count(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, name: &str) -> Option<&Window>;
    fn get_name(&self, id: WindowId) -> Option<&str>;
    fn insert(&mut self, name: String, window: Window);
    fn drop_id(&mut self, id: WindowId);
    fn drop_name(&mut self, name: &str);
    fn render(&mut self, id: WindowId, pearls: &mut BobaPearls, resources: &mut BobaResources);
}

pub trait RenderBuilder {
    type Renderer: MilkTeaRenderer;
    fn build(self, window: Window) -> Self::Renderer;
}

pub struct MilkTeaWindows {
    renderer: Box<dyn MilkTeaRenderer>,
    window_queue: IndexMap<String, WindowBuilder>,
}

impl MilkTeaWindows {
    pub(super) fn new(renderer: impl MilkTeaRenderer) -> Self {
        Self {
            renderer: Box::new(renderer),
            window_queue: IndexMap::new(),
        }
    }

    pub(super) fn render(
        &mut self,
        id: WindowId,
        pearls: &mut BobaPearls,
        resources: &mut BobaResources,
    ) {
        self.renderer.render(id, pearls, resources);
    }

    pub(super) fn build_window_queue(&mut self, event_loop: &EventLoopWindowTarget<()>) {
        for (name, builder) in self.window_queue.drain(..) {
            match builder.build(event_loop) {
                Err(error) => error!("Failed to create window `{name}`. Error: {error}"),
                Ok(window) => {
                    window.request_redraw();
                    self.renderer.insert(name, window);
                }
            };
        }
    }

    pub(super) fn drop_id(&mut self, id: WindowId) {
        self.renderer.drop_id(id);
    }

    pub fn window_count(&self) -> usize {
        self.renderer.window_count()
    }

    pub fn is_empty(&self) -> bool {
        self.renderer.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&Window> {
        self.renderer.get(name)
    }

    pub fn get_name(&self, id: WindowId) -> Option<&str> {
        self.renderer.get_name(id)
    }

    pub fn insert(&mut self, name: &str, builder: WindowBuilder) {
        self.window_queue.insert(name.into(), builder);
    }

    pub fn drop(&mut self, name: &str) {
        self.window_queue.remove(name);
        self.renderer.drop_name(name);
    }
}
