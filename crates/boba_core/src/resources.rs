use std::time::Instant;

use anymap::AnyMap;

pub struct BobaTime {
    delta: f64,
    unscaled_delta: f64,
    time_scale: f64,
    instant: Instant,
}

impl Default for BobaTime {
    fn default() -> Self {
        Self {
            delta: 0.,
            unscaled_delta: 0.,
            time_scale: 1.,
            instant: Instant::now(),
        }
    }
}

impl BobaTime {
    pub(crate) fn reset(&mut self) {
        self.unscaled_delta = self.instant.elapsed().as_secs_f64();
        self.delta = self.unscaled_delta * self.time_scale;
        self.instant = Instant::now();
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }

    pub fn unscaled_delta(&self) -> f64 {
        self.unscaled_delta
    }

    pub fn set_time_scale(&mut self, time_scale: f64) {
        self.time_scale = time_scale
    }
}

pub struct BobaResources {
    time: BobaTime,
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

    pub fn time_mut(&mut self) -> &mut BobaTime {
        &mut self.time
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
