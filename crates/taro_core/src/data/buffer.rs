use wgpu::util::DeviceExt;

use crate::{BindingCompiler, Compiler, Taro, TaroHardware};

pub trait BytesBuilder: Default + 'static {
    const LABEL: &'static str;
    fn build_bytes(&self) -> &[u8];
}

pub trait BufferBuilder: BytesBuilder {
    const BUFFER_TYPE: wgpu::BufferBindingType;
    const FORCE_USAGES: wgpu::BufferUsages;
}

pub struct Buffer<T: BufferBuilder> {
    default: T,
    usage: wgpu::BufferUsages,
}

impl<T: BufferBuilder> Buffer<T> {
    pub fn new(usage: wgpu::BufferUsages) -> Taro<Self> {
        Self::new_with_default(usage, T::default())
    }

    pub fn new_with_default(usage: wgpu::BufferUsages, default: T) -> Taro<Self> {
        Taro::new(Self { usage, default })
    }
}

impl<T: BufferBuilder> Taro<Buffer<T>> {
    pub fn write_to_hardware(&self, data: T, hardware: &TaroHardware) {
        hardware
            .queue()
            .write_buffer(self.get_or_compile(hardware), 0, data.build_bytes())
    }
}

impl<T: BufferBuilder> Compiler for Buffer<T> {
    type Compiled = wgpu::Buffer;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let label = T::LABEL;
        hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label} Buffer")),
                contents: self.default.build_bytes(),
                usage: self.usage,
            })
    }
}

impl<T: BufferBuilder> BindingCompiler for Taro<Buffer<T>> {
    const LABEL: &'static str = T::LABEL;
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

pub struct Uniform<T: BytesBuilder> {
    data: T,
}

impl<T: BytesBuilder> From<T> for Uniform<T> {
    fn from(data: T) -> Self {
        Uniform { data }
    }
}

impl<T: BytesBuilder> Default for Uniform<T> {
    fn default() -> Self {
        Self { data: T::default() }
    }
}

impl<T: BytesBuilder> BytesBuilder for Uniform<T> {
    const LABEL: &'static str = T::LABEL;
    fn build_bytes(&self) -> &[u8] {
        self.data.build_bytes()
    }
}

impl<T: BytesBuilder> BufferBuilder for Uniform<T> {
    const BUFFER_TYPE: wgpu::BufferBindingType = wgpu::BufferBindingType::Uniform;
    const FORCE_USAGES: wgpu::BufferUsages = wgpu::BufferUsages::UNIFORM;
}

pub struct Storage<T: BytesBuilder, const READONLY: bool> {
    data: T,
}

impl<T: BytesBuilder, const READONLY: bool> From<T> for Storage<T, READONLY> {
    fn from(data: T) -> Self {
        Storage { data }
    }
}

impl<T: BytesBuilder, const READONLY: bool> Default for Storage<T, READONLY> {
    fn default() -> Self {
        Self { data: T::default() }
    }
}

impl<T: BytesBuilder, const READONLY: bool> BytesBuilder for Storage<T, READONLY> {
    const LABEL: &'static str = T::LABEL;
    fn build_bytes(&self) -> &[u8] {
        self.data.build_bytes()
    }
}

impl<T: BytesBuilder, const READONLY: bool> BufferBuilder for Storage<T, READONLY> {
    const FORCE_USAGES: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE;
    const BUFFER_TYPE: wgpu::BufferBindingType = wgpu::BufferBindingType::Storage {
        read_only: READONLY,
    };
}
