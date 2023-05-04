use boba_core::{BobaPearls, BobaResources};
use indexmap::{IndexMap, IndexSet};
use winit::{event_loop::EventLoopWindowTarget, window::WindowId};

use crate::events::{WindowClosed, WindowSpawned};

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Milk Tea Window".into(),
            size: (1280, 800),
        }
    }
}

pub trait RenderBuilder {
    type Renderer: RenderManager;
    fn build(
        self,
        name: &str,
        settings: WindowSettings,
        target: &EventLoopWindowTarget<()>,
    ) -> anyhow::Result<Self::Renderer>;
}

pub trait RenderManager: 'static {
    // window management
    fn spawn_window(
        &mut self,
        name: &str,
        settings: WindowSettings,
        target: &EventLoopWindowTarget<()>,
    ) -> anyhow::Result<()>;
    fn close_window(&mut self, name: &str) -> bool;
    fn get_window(&mut self, name: &str) -> Option<&mut dyn WindowEditor>;

    // system calls
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get_name(&self, id: &WindowId) -> Option<&str>;
    fn redraw(&mut self, id: &WindowId, pearls: &mut BobaPearls, resources: &mut BobaResources);
}

pub struct Windows {
    renderer: Box<dyn RenderManager>,
    close_queue: IndexSet<String>,
    spawn_queue: IndexMap<String, WindowSettings>,
}

impl Windows {
    pub(crate) fn new(renderer: impl RenderManager) -> Self {
        Self {
            renderer: Box::new(renderer),
            close_queue: IndexSet::new(),
            spawn_queue: IndexMap::new(),
        }
    }

    pub(crate) fn spawn_now(
        &mut self,
        name: &str,
        settings: WindowSettings,
        target: &EventLoopWindowTarget<()>,
    ) -> anyhow::Result<WindowSpawned> {
        self.renderer.spawn_window(&name, settings, target)?;
        Ok(WindowSpawned { name: name.into() })
    }

    pub(crate) fn close_now(&mut self, name: &str) -> Option<WindowClosed> {
        match self.renderer.close_window(name) {
            false => None,
            true => Some(WindowClosed { name: name.into() }),
        }
    }

    pub(crate) fn submit_queues(
        &mut self,
        target: &EventLoopWindowTarget<()>,
    ) -> (Vec<WindowClosed>, Vec<WindowSpawned>) {
        let mut closed = Vec::new();
        let mut spawned = Vec::new();

        let closes: Vec<_> = self.close_queue.drain(..).collect();
        let spawns: Vec<_> = self.spawn_queue.drain(..).collect();

        for name in closes {
            match self.close_now(&name) {
                Some(event) => closed.push(event),
                None => log::warn!("Tried closing non-existent window '{name}'"),
            }
        }

        for (name, settings) in spawns {
            match self.spawn_now(&name, settings, target) {
                Ok(event) => spawned.push(event),
                Err(e) => log::error!("Error when spawning window '{name}': {e}"),
            }
        }

        (closed, spawned)
    }

    pub(crate) fn manager(&mut self) -> &mut dyn RenderManager {
        &mut *self.renderer
    }

    pub fn get_window(&mut self, name: &str) -> Option<&mut dyn WindowEditor> {
        self.renderer.get_window(name)
    }

    pub fn queue_spawn(&mut self, name: &str, settings: WindowSettings) {
        self.spawn_queue.insert(name.into(), settings);
    }

    pub fn queue_close(&mut self, name: &str) {
        self.close_queue.insert(name.into());
    }
}

pub trait WindowEditor: 'static {
    // ---
    // required methods
    fn title(&self) -> String;
    fn set_title(&mut self, title: &str);
    fn size(&self) -> (u32, u32);
    fn set_size(&mut self, width: u32, height: u32);
    fn position(&self) -> (u32, u32);
    fn set_position(&self, x: u32, y: u32);
    fn fullscreen(&self) -> bool;
    fn set_fullscreen(&mut self, full: bool);

    // ---
    // auto-implemented helper methods
    fn move_position(&mut self, delta: (u32, u32)) {
        let (x, y) = self.position();
        self.set_position(x + delta.0, y + delta.1);
    }

    fn expand(&mut self, delta_x: i32, delta_y: i32) {
        let size = self.size();
        let new_x = (size.0 as i64 + delta_x as i64).clamp(0, u32::MAX as i64) as u32;
        let new_y = (size.1 as i64 + delta_y as i64).clamp(0, u32::MAX as i64) as u32;
        self.set_size(new_x, new_y);
    }
}
