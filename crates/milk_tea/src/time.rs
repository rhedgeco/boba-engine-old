use std::time::Instant;

pub struct MilkTeaTime {
    instant: Option<Instant>,
    delta_time: f64,
}

impl MilkTeaTime {
    pub(crate) fn new() -> Self {
        Self {
            instant: None,
            delta_time: 0.,
        }
    }

    pub(crate) fn reset(&mut self) -> f64 {
        match self.instant {
            None => self.delta_time = 0.,
            Some(instant) => self.delta_time = instant.elapsed().as_secs_f64(),
        }

        self.instant = Some(Instant::now());
        self.delta_time
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
