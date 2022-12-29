use std::ops::Deref;

use milk_tea::{winit::window::Window, RenderAdapter};

use crate::{SurfaceSize, TaroRenderer};

pub struct TaroMilkTea {
    window: Window,
    renderer: TaroRenderer,
}

impl Deref for TaroMilkTea {
    type Target = TaroRenderer;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}

impl RenderAdapter for TaroMilkTea {
    fn build(window: Window) -> Self {
        let size = window.inner_size();
        let renderer = pollster::block_on(TaroRenderer::new(
            &window,
            SurfaceSize {
                width: size.width,
                height: size.height,
            },
        ));

        Self { window, renderer }
    }

    fn raw_window(&self) -> &Window {
        &self.window
    }
}
