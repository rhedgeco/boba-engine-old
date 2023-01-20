use wgpu::util::DeviceExt;

use crate::{BindingCompiler, Compiler, Taro, TaroHardware};

/// Required for data to be uploaded to the GPU
pub trait BytesBuilder: Default + 'static {
    const LABEL: &'static str;
    fn build_bytes(&self) -> &[u8];
}

/// Required for structs to be built into a [`Buffer`] object
pub trait BufferBuilder: BytesBuilder {
    const BUFFER_TYPE: wgpu::BufferBindingType;
    const FORCE_USAGES: wgpu::BufferUsages;
}

/// Struct for managing buffers to be uploaded to the GPU
pub struct Buffer<T: BufferBuilder> {
    default: T,
    usage: wgpu::BufferUsages,
}

impl<T: BufferBuilder> Buffer<T> {
    /// Creates a new buffer
    pub fn new(usage: wgpu::BufferUsages) -> Taro<Self> {
        Self::new_with_default(usage, T::default())
    }

    /// Creates a new buffer with a default value
    ///
    /// Every time new hardware compiles the buffer, the `default` value will be used.
    pub fn new_with_default(usage: wgpu::BufferUsages, default: T) -> Taro<Self> {
        Taro::new(Self { usage, default })
    }
}

impl<T: BufferBuilder> Taro<Buffer<T>> {
    /// Writes data to the compiled buffer associated with `hardware`
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
        let usage =
            wgpu::BufferUsages::COPY_DST.bits() | T::FORCE_USAGES.bits() | self.usage.bits();
        let usage = wgpu::BufferUsages::from_bits_truncate(usage);
        hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label} Buffer")),
                contents: self.default.build_bytes(),
                usage,
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

/// Represents uniform buffer data
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

/// Represents storage buffer data
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
