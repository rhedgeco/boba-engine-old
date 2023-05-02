use boba_core::Event;

pub struct Update {
    delta_time: f64,
}

impl Event for Update {
    type Data<'a> = Self;
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

impl Event for LateUpdate {
    type Data<'a> = Self;
}

impl LateUpdate {
    pub(crate) fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
