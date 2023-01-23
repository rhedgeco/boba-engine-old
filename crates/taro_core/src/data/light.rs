use boba_3d::{glam::Vec3, pearls::BobaTransform};
use boba_core::Pearl;
use log::error;
use wgpu::Color;

use crate::{Bind, Taro, TaroHardware};

use super::{buffers, Buffer, UniformBinding};

pub struct PointLight {
    pub transform: Pearl<BobaTransform>,
    pub color: Color,

    light_binding: Taro<UniformBinding<buffers::PointLight>>,
}

impl PointLight {
    pub fn new_simple(position: Vec3, color: Color) -> Self {
        let transform = Pearl::wrap(BobaTransform::from_position(position));
        Self::new(transform, color)
    }

    pub fn new(transform: Pearl<BobaTransform>, color: Color) -> Self {
        let light = match transform.borrow() {
            Ok(t) => Buffer::new_with_default(
                wgpu::BufferUsages::empty(),
                buffers::PointLight::new(t.world_position(), color).into(),
            ),
            Err(e) => {
                error!("Error when getting light position. Error: {e}");
                Buffer::new(wgpu::BufferUsages::empty().into())
            }
        };

        Self {
            transform,
            color,
            light_binding: Bind::new(light),
        }
    }

    pub fn recalculate_light_binding(
        &self,
        hardware: &TaroHardware,
    ) -> &Taro<UniformBinding<buffers::PointLight>> {
        match self.transform.borrow() {
            Ok(t) => {
                self.light_binding.get_bind_data().write_to_hardware(
                    buffers::PointLight::new(t.world_position(), self.color).into(),
                    hardware,
                );
            }
            Err(e) => {
                error!("Error when calculating light binding. Error: {e}");
            }
        };

        &self.light_binding
    }
}
