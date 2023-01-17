use once_map::OnceMap;
use wgpu::util::DeviceExt;

use crate::{
    binding::{Bind, BindingCompiler, CompiledSingleBinding},
    Compiler, Taro, TaroHardware,
};

/// Required for objects to be built into a [`Buffer`] and uploaded to GPU
pub trait BytesBuilder: Default + 'static {
    fn build_bytes(&self) -> &[u8];
}

/// Required to identify binding types when building into a [`Buffer`]
pub trait BufferCompiler: BytesBuilder {
    const BUFFER_BIND_TYPE: wgpu::BufferBindingType;
}

/// A type to represent buffer data uploaded to the GPU
pub struct Buffer<T: BufferCompiler> {
    label: String,
    usage: wgpu::BufferUsages,
    single_cache: OnceMap<wgpu::ShaderStages, CompiledSingleBinding<Taro<Buffer<T>>>>,
}

impl<T: BufferCompiler> Compiler for Buffer<T> {
    type Compiled = wgpu::Buffer;

    fn manual_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&self.label),
                contents: T::default().build_bytes(),
                usage: self.usage,
            })
    }
}

impl<T: BufferCompiler> BindingCompiler for Taro<Buffer<T>> {
    const BIND_TYPE: wgpu::BindingType = wgpu::BindingType::Buffer {
        ty: T::BUFFER_BIND_TYPE,
        has_dynamic_offset: false,
        min_binding_size: None,
    };

    fn manual_compile_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        self.get_or_compile(hardware).as_entire_binding()
    }
}

impl<T: BufferCompiler> Buffer<T> {
    /// Create a new buffer wrapped in a [`Taro`] object
    pub fn new(label: String, usage: wgpu::BufferUsages) -> Taro<Buffer<T>> {
        let buffer = Buffer {
            label,
            usage,
            single_cache: Default::default(),
        };

        Taro::new(buffer)
    }

    /// Gets the usages available for this buffer
    pub fn usage(&self) -> &wgpu::BufferUsages {
        &self.usage
    }
}

impl<T: BufferCompiler> Taro<Buffer<T>> {
    /// Writes `data` to the buffer associated with the specified `hardware`
    ///
    /// Data in a buffer may be different across hardware destinations.
    /// But if you are using multiple GPUs, I imagine that this is understandable
    /// and I wish you the best of luck lol
    pub fn write_to_hardware(&self, data: &T, hardware: &TaroHardware) -> &wgpu::Buffer {
        let buffer = self.get_or_compile(hardware);
        hardware.queue().write_buffer(buffer, 0, data.build_bytes());
        buffer
    }

    /// Gets or compiles a [`CompiledSingleBinding`] associated with this buffer
    pub fn get_or_compile_single_binding(
        &self,
        visibility: wgpu::ShaderStages,
        hardware: &TaroHardware,
    ) -> &CompiledSingleBinding<Taro<Buffer<T>>> {
        self.single_cache
            .get_or_init(visibility, || {
                Bind::direct_manual_compile(self.clone(), visibility, hardware)
            })
            .into_data()
    }
}
