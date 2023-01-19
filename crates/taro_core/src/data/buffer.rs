use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::{BindingCompiler, Compiler, Taro};

struct InnerBuffer<T: BytesBuilder> {
    usage: wgpu::BufferUsages,
    _type: PhantomData<T>,
}

impl<T: BytesBuilder> Compiler for InnerBuffer<T> {
    type Compiled = wgpu::Buffer;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let label = T::LABEL;
        hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label} Buffer")),
                contents: T::default().build_bytes(),
                usage: self.usage,
            })
    }
}

pub trait BytesBuilder: Default + 'static {
    const LABEL: &'static str;
    fn build_bytes(&self) -> &[u8];
}

pub struct UniformBuffer<T: BytesBuilder> {
    inner: InnerBuffer<T>,
}

impl<T: BytesBuilder> UniformBuffer<T> {
    pub fn new(usage: wgpu::BufferUsages) -> Taro<Self> {
        Taro::new(Self {
            inner: InnerBuffer {
                usage,
                _type: PhantomData,
            },
        })
    }
}

impl<T: BytesBuilder> Compiler for UniformBuffer<T> {
    type Compiled = wgpu::Buffer;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        self.inner.new_taro_compile(hardware)
    }
}

impl<T: BytesBuilder> BindingCompiler for Taro<UniformBuffer<T>> {
    const LABEL: &'static str = "Taro UniformBuffer";
    const COUNT: Option<std::num::NonZeroU32> = None;
    const BIND_TYPE: wgpu::BindingType = wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    };

    fn compile_new_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {
        self.get_or_compile(hardware).as_entire_binding()
    }
}

pub struct StorageBuffer<T: BytesBuilder, const READONLY: bool> {
    inner: InnerBuffer<T>,
}

impl<T: BytesBuilder, const READONLY: bool> StorageBuffer<T, READONLY> {
    pub fn new(usage: wgpu::BufferUsages) -> Taro<Self> {
        Taro::new(Self {
            inner: InnerBuffer {
                usage,
                _type: PhantomData,
            },
        })
    }
}

impl<T: BytesBuilder, const READONLY: bool> Compiler for StorageBuffer<T, READONLY> {
    type Compiled = wgpu::Buffer;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        self.inner.new_taro_compile(hardware)
    }
}

impl<T: BytesBuilder, const READONLY: bool> BindingCompiler for Taro<StorageBuffer<T, READONLY>> {
    const LABEL: &'static str = "Taro StorageBuffer";
    const COUNT: Option<std::num::NonZeroU32> = None;
    const BIND_TYPE: wgpu::BindingType = wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage {
            read_only: READONLY,
        },
        has_dynamic_offset: false,
        min_binding_size: None,
    };

    fn compile_new_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {
        self.get_or_compile(hardware).as_entire_binding()
    }
}
