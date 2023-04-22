pub struct MilkTeaSettings {
    pub exit_when_close_requested: bool,
}

impl Default for MilkTeaSettings {
    fn default() -> Self {
        Self {
            exit_when_close_requested: true,
        }
    }
}
