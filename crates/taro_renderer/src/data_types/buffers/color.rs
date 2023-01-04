use crate::data_types::TaroBytesBuilder;

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
    pub const WHITE: Color = Color {
        values: [1., 1., 1., 1.],
    };

    pub const BLACK: Color = Color {
        values: [0., 0., 0., 1.],
    };

    pub const RED: Color = Color {
        values: [1., 0., 0., 1.],
    };

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            values: [r, g, b, a],
        }
    }
}
