use taro_renderer::wgpu::Color;

pub enum TaroSkybox {
    Color { r: f64, g: f64, b: f64 },
    None,
}

impl TaroSkybox {
    pub(crate) fn wgpu_color(&self) -> Color {
        match self {
            TaroSkybox::Color { r, g, b } => Color {
                r: *r,
                g: *g,
                b: *b,
                a: 1.0,
            },
            TaroSkybox::None => Color::BLACK,
        }
    }
}
