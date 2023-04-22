pub struct MilkTeaSettings {
    pub exit_when_close_requested: bool,
    pub exit_when_no_windows_open: bool,
    pub close_window_when_requested: bool,
}

impl Default for MilkTeaSettings {
    fn default() -> Self {
        Self {
            exit_when_close_requested: true,
            exit_when_no_windows_open: true,
            close_window_when_requested: true,
        }
    }
}
