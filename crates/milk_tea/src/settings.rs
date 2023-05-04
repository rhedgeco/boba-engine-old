pub struct MilkTeaSettings {
    pub manual_window_redraw: bool,
    pub exit_when_close_requested: bool,
    pub close_window_when_requested: bool,
}

impl Default for MilkTeaSettings {
    fn default() -> Self {
        Self {
            manual_window_redraw: false,
            exit_when_close_requested: true,
            close_window_when_requested: true,
        }
    }
}
