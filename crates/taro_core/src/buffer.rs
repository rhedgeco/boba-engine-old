use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::{Compiler, Taro, TaroHardware};

/// Base trait for an object to be built into a [`Buffer`]
pub trait BufferBuilder<const SIZE: usize>: Default + 'static {
    fn build_bytes(&self) -> &[u8; SIZE];
}

/// A type to represent buffer data uploaded to the GPU
pub struct Buffer<T: BufferBuilder<SIZE>, const SIZE: usize> {
    label: String,
    usage: wgpu::BufferUsages,
    _type: PhantomData<T>,
}

impl<T: BufferBuilder<SIZE>, const SIZE: usize> Compiler for Buffer<T, SIZE> {
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

impl<T: BufferBuilder<SIZE>, const SIZE: usize> Buffer<T, SIZE> {
    /// Create a new buffer wrapped in a [`Taro`] object
    pub fn new(label: String, usage: wgpu::BufferUsages) -> Taro<Buffer<T, SIZE>> {
        let buffer = Buffer {
            label,
            usage,
            _type: PhantomData,
        };

        Taro::new(buffer)
    }

    /// Gets the usages available for this buffer
    pub fn usage(&self) -> &wgpu::BufferUsages {
        &self.usage
    }
}

impl<T: BufferBuilder<SIZE>, const SIZE: usize> Taro<Buffer<T, SIZE>> {
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
}
