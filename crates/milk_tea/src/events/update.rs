use std::time::Instant;

use boba_core::Event;

pub struct Update {
    pub delta: f64,
}

impl Event for Update {
    type Data<'a> = &'a Self;
}

pub struct LateUpdate {
    pub delta: f64,
}

impl Event for LateUpdate {
    type Data<'a> = &'a Self;
}

pub struct Time {
    instant: Option<Instant>,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self { instant: None }
    }

    pub(crate) fn reset_delta(&mut self) -> f64 {
        let delta = self.delta();
        self.instant = Some(Instant::now());
        delta
    }

    pub fn delta(&self) -> f64 {
        match self.instant {
            None => 0.,
            Some(instant) => instant.elapsed().as_secs_f64(),
        }
    }
}
