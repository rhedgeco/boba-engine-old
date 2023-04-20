use boba_core::{pearls::map::BobaPearls, BobaResources};
use indexmap::{IndexMap, IndexSet};
use log::error;
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

use crate::events::WindowSpawn;

pub trait RenderBuilder {
    type Renderer: MilkTeaRenderer;
    fn build(self, window: Window) -> Self::Renderer;
}

pub trait MilkTeaRenderer: 'static {
    fn window_count(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, name: &str) -> Option<&Window>;
    fn get_name(&self, id: WindowId) -> Option<&str>;
    fn insert(&mut self, name: String, window: Window);
    fn drop_by_name(&mut self, name: String);
    fn drop_by_id(&mut self, id: WindowId) -> Option<String>;
    fn render(&mut self, id: WindowId, pearls: &mut BobaPearls, resources: &mut BobaResources);
}

pub struct MilkTeaWindows {
    renderer: Box<dyn MilkTeaRenderer>,
    insert_queue: IndexMap<String, WindowBuilder>,
    name_drop_queue: IndexSet<String>,
}

impl MilkTeaWindows {
    pub(super) fn new(renderer: impl MilkTeaRenderer) -> Self {
        Self {
            renderer: Box::new(renderer),
            insert_queue: IndexMap::new(),
            name_drop_queue: IndexSet::new(),
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

    pub(super) fn spawn_next(
        &mut self,
        window_target: &EventLoopWindowTarget<()>,
    ) -> Option<WindowSpawn> {
        let (name, builder) = self.insert_queue.pop()?;
        let window = match builder.build(window_target) {
            Ok(window) => window,
            Err(error) => {
                error!("Failed to create window `{name}`. Error: {error}");
                return self.spawn_next(window_target);
            }
        };

        let spawn_event = WindowSpawn::new(window.id(), name.clone());
        window.request_redraw();
        self.name_drop_queue.remove(&name);
        self.renderer.insert(name, window);

        Some(spawn_event)
    }

    pub(super) fn submit_drop_queue(&mut self) {
        for name in self.name_drop_queue.drain(..) {
            self.renderer.drop_by_name(name);
        }
    }

    pub(super) fn drop_now(&mut self, id: WindowId) -> Option<String> {
        self.renderer.drop_by_id(id)
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
        self.insert_queue.insert(name.into(), builder);
    }

    pub fn queue_drop(&mut self, name: &str) {
        self.insert_queue.remove(name);
        self.name_drop_queue.insert(name.to_string());
    }
}
