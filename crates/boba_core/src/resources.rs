use anymap::AnyMap;

pub struct BobaResources {
    resources: AnyMap,
}

impl Default for BobaResources {
    fn default() -> Self {
        Self {
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
}
