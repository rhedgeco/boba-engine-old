pub struct Update {
    pub delta_time: f64,
}

impl Update {
    pub fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }
}
