pub struct MilkTeaUpdate {
    pub delta_time: f64,
}

impl MilkTeaUpdate {
    pub fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }
}
