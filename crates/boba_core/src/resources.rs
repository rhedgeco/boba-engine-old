use std::time::Instant;

use anymap::AnyMap;

pub struct BobaTime {
    delta: f64,
    instant: Instant,
}

impl Default for BobaTime {
    fn default() -> Self {
        Self {
            delta: Default::default(),
            instant: Instant::now(),
        }
    }
}

impl BobaTime {
    pub(crate) fn reset(&mut self) {
        self.delta = self.instant.elapsed().as_secs_f64();
        self.instant = Instant::now();
    }

    pub fn delta(&self) -> &f64 {
        &self.delta
    }
}

pub struct BobaResources {
    pub(crate) time: BobaTime,
    resources: AnyMap,
}

impl Default for BobaResources {
    fn default() -> Self {
        Self {
            time: Default::default(),
            resources: AnyMap::new(),
        }
    }
}

impl BobaResources {
    pub fn time(&self) -> &BobaTime {
        &self.time
    }

    pub fn add<T: 'static>(&mut self, item: T) {
        self.resources.insert(item);
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
}
