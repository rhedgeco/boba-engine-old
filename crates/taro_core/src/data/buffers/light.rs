use boba_3d::glam::Vec3;
use wgpu::Color;

use crate::data::BytesBuilder;

/// Local representation of point light data that can be uploaded to a GPU buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLight {
    values: [[f32; 4]; 2],
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            values: [[0.; 4]; 2],
        }
    }
}

impl BytesBuilder for PointLight {
    const LABEL: &'static str = "PointLight";
    fn build_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.values)
    }
}

impl PointLight {
    /// Creates a new point light from a position and color
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {
            values: [
                [position.x, position.y, position.z, 0.],
                [
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ],
            ],
        }
    }
}
