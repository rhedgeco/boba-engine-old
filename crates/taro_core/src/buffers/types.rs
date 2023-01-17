use crate::{BufferCompiler, BytesBuilder};

pub struct Uniform<T: BytesBuilder> {
    data: T,
}

pub struct Storage<T: BytesBuilder, const READONLY: bool> {
    data: T,
}

impl<T: BytesBuilder> Default for Uniform<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T: BytesBuilder, const READONLY: bool> Default for Storage<T, READONLY> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T: BytesBuilder> BytesBuilder for Uniform<T> {
    fn build_bytes(&self) -> &[u8] {
        self.data.build_bytes()
    }
}

impl<T: BytesBuilder, const READONLY: bool> BytesBuilder for Storage<T, READONLY> {
    fn build_bytes(&self) -> &[u8] {
        self.data.build_bytes()
    }
}

impl<T: BytesBuilder> BufferCompiler for Uniform<T> {
    const BUFFER_BIND_TYPE: wgpu::BufferBindingType = wgpu::BufferBindingType::Uniform;
}

impl<T: BytesBuilder, const READONLY: bool> BufferCompiler for Storage<T, READONLY> {
    const BUFFER_BIND_TYPE: wgpu::BufferBindingType = wgpu::BufferBindingType::Storage {
        read_only: READONLY,
    };
}

impl<T: BytesBuilder> From<T> for Uniform<T> {
    fn from(data: T) -> Self {
        Uniform { data }
    }
}

impl<T: BytesBuilder, const READONLY: bool> From<T> for Storage<T, READONLY> {
    fn from(data: T) -> Self {
        Storage { data }
    }
}
