use super::{TaroData, TaroDataUploader};
use crate::{
    data_types::{
        buffers::{CameraMatrix, TransformMatrix},
        TaroMeshBuffer,
    },
    HardwareId, TaroHardware,
};
use once_map::OnceMap;
use std::{any::TypeId, marker::PhantomData};
use wgpu::util::DeviceExt;

/// The base trait for any shader type.
///
/// To be a shader, this *must* be implemented.
pub trait TaroCoreShader: 'static {
    type InitParameters;
    fn build_instance(init: &Self::InitParameters, hardware: &TaroHardware) -> Self;
}

/// The base trait for a shader that can render a mesh on screen.
pub trait TaroMeshShader: TaroCoreShader {
    fn render<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        mesh: &'pass TaroMeshBuffer,
        camera_matrix: &'pass TaroBuffer<CameraMatrix>,
        model_matrix: &'pass TaroBuffer<TransformMatrix>,
        hardware: &TaroHardware,
    );
}

/// The main struct to hold and manage shaders for TaroRenderers
#[derive(Clone)]
pub struct TaroShader<T>
where
    T: TaroCoreShader,
{
    parameters: T::InitParameters,
    shader_cache: OnceMap<HardwareId, T>,
}

impl<T> TaroShader<T>
where
    T: TaroCoreShader,
{
    /// Creates a new TaroShader with `init` parameters
    pub fn new(init: T::InitParameters) -> Self {
        Self {
            parameters: init,
            shader_cache: Default::default(),
        }
    }

    /// Gets the compiled shader associated with `hardware`
    ///
    /// If the shader has not been compiled yet, it will be now.
    pub fn get(&self, hardware: &TaroHardware) -> &T {
        self.shader_cache
            .get_or_init(hardware.id(), || {
                T::build_instance(&self.parameters, hardware)
            })
            .into_data()
    }
}

/// The base trait for structs that can be built into an array of bytes.
///
/// This is useful for when data needs to be uploaded to the GPU
pub trait TaroBytesBuilder {
    fn as_bytes(&self) -> &[u8];
}

/// A buffer manager for uploaded GPU data.
pub struct TaroBuffer<T> {
    hardware_id: HardwareId,
    buffer: wgpu::Buffer,
    binding_cache: OnceMap<TypeId, wgpu::BindGroup>,
    _type: PhantomData<T>,
}

impl<T> TaroDataUploader for T
where
    T: TaroBytesBuilder,
{
    type UploadData = TaroBuffer<T>;

    /// Compiles a new instance of the buffer data using `hardware`
    fn new_upload(&self, hardware: &TaroHardware) -> Self::UploadData {
        TaroBuffer::<T>::new(self, hardware)
    }
}

impl<T> TaroData<T> for TaroBuffer<T>
where
    T: TaroBytesBuilder,
{
    /// Writes `new data` to the internal buffer associated with `hardware`
    ///
    /// # Panics
    /// This will panic if it is written to with different `hardware` than it was created with.
    fn write_new(&self, new_data: &T, hardware: &TaroHardware) {
        if hardware.id() != &self.hardware_id {
            panic!(
                "Tried to write to TaroBuffer<{:?}> with TaroHardware that does not match the original compiler.",
                std::any::type_name::<T>()
            );
        }

        hardware
            .queue()
            .write_buffer(&self.buffer, 0, new_data.as_bytes());
    }
}

impl<T> TaroBuffer<T>
where
    T: TaroBytesBuilder,
{
    /// Creates a new TaroBuffer and initializes a buffer associated with `hardware`
    fn new(data: &T, hardware: &TaroHardware) -> Self {
        Self {
            _type: Default::default(),
            binding_cache: OnceMap::new(),
            hardware_id: hardware.id().clone(),
            buffer: hardware
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{} Buffer", std::any::type_name::<T>())),
                    contents: data.as_bytes(),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }),
        }
    }
}

pub trait ShaderExt {
    fn get_or_init_binding<'a, Data>(
        &'a self,
        buffer: &'a TaroBuffer<Data>,
        layout: &wgpu::BindGroupLayout,
        hardware: &TaroHardware,
    ) -> &wgpu::BindGroup;
}

impl<T> ShaderExt for T
where
    T: TaroCoreShader,
{
    /// Creates or gets a BindGroup associated with the provided `buffer`
    ///
    /// If the BindGroup does not exist for this shader, it will initialize it using `layout`
    ///
    /// # Panics
    /// This will panic if it is retrieved to with different `hardware` than it was created with.
    fn get_or_init_binding<'a, Data>(
        &'a self,
        buffer: &'a TaroBuffer<Data>,
        layout: &wgpu::BindGroupLayout,
        hardware: &TaroHardware,
    ) -> &wgpu::BindGroup {
        if &buffer.hardware_id != hardware.id() {
            panic!(
                "Tried to get TaroBuffer<{:?}> with TaroHardware that does not match the original compiler.",
                std::any::type_name::<Data>()
            );
        }

        let typeid = TypeId::of::<Self>();
        buffer
            .binding_cache
            .get_or_init(&typeid, || {
                hardware
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: buffer.buffer.as_entire_binding(),
                        }],
                        label: Some(&format!("{} BindGroup", std::any::type_name::<Data>())),
                    })
            })
            .into_data()
    }
}

/// A struct to help with managing parameters attached directly to a shader
pub struct ShaderParameter<T> {
    hardware_id: HardwareId,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    _type: PhantomData<T>,
}

impl<T> ShaderParameter<T>
where
    T: TaroBytesBuilder,
{
    /// Gets the internal bind group associated with this parameter
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    /// Creates a new ShaderParameter instance, and initializes it.
    pub fn new(initial_value: &T, layout: &wgpu::BindGroupLayout, hardware: &TaroHardware) -> Self {
        let buffer = hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Buffer", std::any::type_name::<T>())),
                contents: initial_value.as_bytes(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some(&format!("{} BindGroup", std::any::type_name::<T>())),
            });

        Self {
            hardware_id: hardware.id().clone(),
            buffer,
            bind_group,
            _type: Default::default(),
        }
    }

    /// Writes `new_data` to the ShaderParameter
    ///
    /// # Panics
    /// This will panic if it is written to with different `hardware` than it was created with.
    pub fn write(&self, new_data: &T, hardware: &TaroHardware) {
        if hardware.id() != &self.hardware_id {
            panic!(
                "Tried to set ShaderParameter<{:?}> with TaroHardware that does not match the original.",
                std::any::type_name::<T>()
            )
        }

        hardware
            .queue()
            .write_buffer(&self.buffer, 0, new_data.as_bytes());
    }
}
