pub struct MilkTeaSettings {
    pub exit_when_close_requested: bool,
    pub close_window_when_requested: bool,
}

impl Default for MilkTeaSettings {
    fn default() -> Self {
        Self {
            exit_when_close_requested: true,
            close_window_when_requested: true,
        }
    }
}
