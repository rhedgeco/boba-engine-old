use std::ops::Deref;

use winit::window::Window;

pub struct MilkTeaWindow<Renderer>
where
    Renderer: WindowRenderer,
{
    window: Window,
    renderer: Renderer,
}

impl<Renderer> Deref for MilkTeaWindow<Renderer>
where
    Renderer: WindowRenderer,
{
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl<Renderer> MilkTeaWindow<Renderer>
where
    Renderer: WindowRenderer,
{
    pub fn new(window: Window) -> Self {
        let renderer = Renderer::build(&window);

        Self { window, renderer }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}

pub trait WindowRenderer: 'static {
    fn build(window: &Window) -> Self;
    fn render(&self);
}
