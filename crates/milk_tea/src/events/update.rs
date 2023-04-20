pub struct Update {
    delta_time: f64,
}

impl Update {
    pub(crate) fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}

pub struct LateUpdate {
    delta_time: f64,
}

impl LateUpdate {
    pub(crate) fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
