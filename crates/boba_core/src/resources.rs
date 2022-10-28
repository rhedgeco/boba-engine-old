use anymap::AnyMap;

pub enum AppState {
    Running,
    Stopping,
}

pub struct BobaResources {
    app_state: AppState,
    resources: AnyMap,
}

impl Default for BobaResources {
    fn default() -> Self {
        Self {
            app_state: AppState::Running,
            resources: AnyMap::new(),
        }
    }
}

impl BobaResources {
    pub fn insert<T: 'static>(&mut self, item: T) {
        self.resources.insert(item);
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn app_state(&self) -> &AppState {
        &self.app_state
    }

    pub fn quit_app(&mut self) {
        self.app_state = AppState::Stopping
    }
}
