use winit::window::Window;

pub struct MilkTeaWindows {
    main: Window,
}

impl MilkTeaWindows {
    pub fn new(main_window: Window) -> Self {
        Self { main: main_window }
    }

    pub fn main(&self) -> &Window {
        &self.main
    }

    pub fn main_mut(&mut self) -> &mut Window {
        &mut self.main
    }
}
