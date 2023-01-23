use std::sync::Arc;

use crate::{
    data::{
        buffers::Color,
        texture::{Simple, Texture2DView},
        Buffer, Sampler, UniformBuffer,
    },
    Bind, BindGroup, BindGroupBuilder, Taro,
};

pub struct LitShader {
    albedo: Taro<Texture2DView<Simple>>,
    albedo_sampler: Taro<Sampler>,
    color: Taro<UniformBuffer<Color>>,
    bindings: Taro<BindGroup>,
}

impl LitShader {
    /// Creates a new unlit shader
    pub fn new(color: Color, albedo: Taro<Texture2DView<Simple>>) -> Arc<Self> {
        let albedo_sampler = Sampler::new();
        let color: Taro<UniformBuffer<Color>> =
            Buffer::new_with_default(wgpu::BufferUsages::empty(), color.into());
        let bindings = BindGroupBuilder::new(0, Bind::new(albedo_sampler.clone()))
            .insert(1, Bind::new(albedo.clone()))
            .insert(2, Bind::new(color.clone()))
            .build();

        Arc::new(Self {
            albedo,
            albedo_sampler,
            color,
            bindings,
        })
    }

    /// Get the albedo texture for this shader
    pub fn albedo(&self) -> &Taro<Texture2DView<Simple>> {
        &self.albedo
    }

    /// Get the texture sampler for this shader
    pub fn albedo_sampler(&self) -> &Taro<Sampler> {
        &self.albedo_sampler
    }

    /// Get the color buffer for this shader
    pub fn color(&self) -> &Taro<UniformBuffer<Color>> {
        &self.color
    }

    /// Gets the bindings for this shader
    ///
    /// The bind group is in this order `0 => sampler` ` 1 => texture_view` ` 2 => color`
    pub fn bindings(&self) -> &Taro<BindGroup> {
        &self.bindings
    }
}
