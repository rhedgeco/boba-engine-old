use boba_core::{pearls::map::BobaPearls, BobaResources};
use indexmap::{IndexMap, IndexSet};
use log::error;
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

use crate::events::{WindowDestroy, WindowSpawn};

pub trait RenderBuilder: Sized {
    type Renderer: WindowRenderer;
    fn build(self) -> Self::Renderer;
}

pub trait WindowRenderer: 'static {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn contains(&self, id: WindowId) -> bool;
    fn init(&mut self, window: Window) -> anyhow::Result<()>;
    fn destroy(&mut self, id: WindowId) -> bool;
    fn render(
        &mut self,
        id: WindowId,
        name: String,
        pearls: &mut BobaPearls,
        resources: &mut BobaResources,
    );
}

pub struct MilkTeaWindows {
    renderer: Box<dyn WindowRenderer>,
    name_to_id: IndexMap<String, WindowId>,
    id_to_name: IndexMap<WindowId, String>,
    spawn_queue: IndexMap<String, WindowBuilder>,
    destroy_queue: IndexSet<String>,
}

impl MilkTeaWindows {
    pub(crate) fn new(renderer: impl WindowRenderer) -> Self {
        Self {
            renderer: Box::new(renderer),
            name_to_id: IndexMap::new(),
            id_to_name: IndexMap::new(),
            spawn_queue: IndexMap::new(),
            destroy_queue: IndexSet::new(),
        }
    }

    pub(crate) fn spawn_now(&mut self, name: &str, window: Window) -> anyhow::Result<WindowSpawn> {
        let window_id = window.id();
        self.renderer.init(window)?;
        self.name_to_id.insert(name.into(), window_id);
        self.id_to_name.insert(window_id, name.into());
        Ok(WindowSpawn::new(name.into()))
    }

    pub(crate) fn submit_destroy_queue(&mut self) -> Vec<WindowDestroy> {
        let mut destroy_events = Vec::new();
        for name in self.destroy_queue.drain(..) {
            let Some(id) = self.name_to_id.get(&name) else { continue };
            if self.renderer.destroy(*id) {
                destroy_events.push(WindowDestroy::new(name));
            }
        }
        destroy_events
    }

    pub(crate) fn submit_spawn_queue(
        &mut self,
        window_target: &EventLoopWindowTarget<()>,
    ) -> Vec<WindowSpawn> {
        let mut spawn_events = Vec::new();
        for (name, builder) in self.spawn_queue.drain(..) {
            let window = match builder.build(window_target) {
                Ok(window) => window,
                Err(e) => {
                    error!("Spawn Window '{name}' Error: {e}");
                    continue;
                }
            };

            let window_id = window.id();
            if let Err(e) = self.renderer.init(window) {
                error!("Window '{name}' renderer init Error: {e}");
                continue;
            }

            self.name_to_id.insert(name.clone(), window_id);
            self.id_to_name.insert(window_id, name.clone());
            spawn_events.push(WindowSpawn::new(name));
        }

        spawn_events
    }

    pub(crate) fn render(
        &mut self,
        id: WindowId,
        pearls: &mut BobaPearls,
        resources: &mut BobaResources,
    ) {
        let Some(name) = self.get_name(id) else { return };
        self.renderer.render(id, name, pearls, resources);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.renderer.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.renderer.len()
    }

    #[inline]
    pub fn contains(&self, name: &str) -> bool {
        let Some(id) = self.get_id(name) else { return false };
        self.renderer.contains(id)
    }

    #[inline]
    pub fn get_id(&self, name: &str) -> Option<WindowId> {
        Some(*self.name_to_id.get(name)?)
    }

    #[inline]
    pub fn get_name(&self, id: WindowId) -> Option<String> {
        Some(self.id_to_name.get(&id)?.clone())
    }

    #[inline]
    pub fn queue_spawn(&mut self, name: &str, builder: WindowBuilder) {
        self.spawn_queue.insert(name.into(), builder);
    }

    #[inline]
    pub fn queue_destroy(&mut self, name: &str) {
        self.destroy_queue.insert(name.into());
    }
}
