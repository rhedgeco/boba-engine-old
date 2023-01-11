use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::{
    shading::{Taro, TaroBindBuilder, TaroBuilder},
    TaroHardware,
};

pub trait TaroBytesBuilder: Default + 'static {
    fn build_bytes(&self) -> &[u8];
}

pub struct UniformBuffer<T: TaroBytesBuilder> {
    _type: PhantomData<T>,
}

impl<T: TaroBytesBuilder> UniformBuffer<T> {
    pub fn new() -> Taro<Self> {
        Default::default()
    }
}

impl<T: TaroBytesBuilder> Default for Taro<UniformBuffer<T>> {
    fn default() -> Self {
        Taro::new(UniformBuffer {
            _type: Default::default(),
        })
    }
}

impl<T: TaroBytesBuilder> TaroBuilder for UniformBuffer<T> {
    type Compiled = wgpu::Buffer;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("UniformBuffer<{}>", std::any::type_name::<T>())),
                contents: T::default().build_bytes(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }
}

impl<T: TaroBytesBuilder> TaroBindBuilder for Taro<UniformBuffer<T>> {
    fn build_bind_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        self.get_or_compile(hardware).as_entire_binding()
    }
}

impl<T: TaroBytesBuilder> Taro<UniformBuffer<T>> {
    pub fn write_buffer(&self, data: &T, hardware: &TaroHardware) -> &wgpu::Buffer {
        let buffer = self.get_or_compile(hardware);
        hardware
            .queue()
            .write_buffer(&buffer, 0, data.build_bytes());
        buffer
    }
}
