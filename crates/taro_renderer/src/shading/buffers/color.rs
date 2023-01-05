use crate::shading::TaroBytesBuilder;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub values: [f32; 4],
}

impl Default for Color {
    fn default() -> Self {
        Self { values: [1.; 4] }
    }
}

impl TaroBytesBuilder for Color {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.values)
    }
}

impl Color {
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            values: [r, g, b, a],
        }
    }
}

impl From<wgpu::Color> for Color {
    fn from(value: wgpu::Color) -> Self {
        Color {
            values: [
                value.r as f32,
                value.g as f32,
                value.b as f32,
                value.a as f32,
            ],
        }
    }
}

impl From<&wgpu::Color> for Color {
    fn from(value: &wgpu::Color) -> Self {
        Color {
            values: [
                value.r as f32,
                value.g as f32,
                value.b as f32,
                value.a as f32,
            ],
        }
    }
}
