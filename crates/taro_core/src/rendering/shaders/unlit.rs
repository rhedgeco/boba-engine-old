use std::sync::Arc;

use crate::{
    data::{
        buffers::Color,
        texture::{Simple, Texture2DView},
        Buffer, Sampler, UniformBuffer,
    },
    Bind, BindGroup, BindGroupBuilder, Taro,
};

pub struct UnlitShader {
    bindings: Taro<BindGroup>,
}

impl UnlitShader {
    /// Creates a new unlit shader
    pub fn new(color: Color, texture_view: Taro<Texture2DView<Simple>>) -> Arc<Self> {
        let sampler = Sampler::new();
        let color_buffer: Taro<UniformBuffer<Color>> =
            Buffer::new_with_default(wgpu::BufferUsages::empty(), color.into());
        let bindings = BindGroupBuilder::new(0, Bind::new(sampler))
            .insert(1, Bind::new(texture_view))
            .insert(2, Bind::new(color_buffer))
            .build();

        Arc::new(Self { bindings })
    }

    /// Gets the bindings for this shader
    ///
    /// The bind group is in this order `0 => sampler` ` 1 => texture_view` ` 2 => color`
    pub fn bindings(&self) -> &Taro<BindGroup> {
        &self.bindings
    }
}
