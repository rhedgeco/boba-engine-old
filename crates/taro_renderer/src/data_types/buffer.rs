use std::{
    any::{type_name, TypeId},
    marker::PhantomData,
};

use once_map::OnceMap;
use wgpu::util::DeviceExt;

use crate::{
    shading::{TaroCoreShader, TaroData, TaroDataUploader},
    HardwareId, TaroHardware,
};

pub trait TaroBytesBuilder {
    fn as_bytes(&self) -> &[u8];
}

pub struct TaroBuffer<T> {
    buffer: wgpu::Buffer,
    binding_cache: OnceMap<TypeId, wgpu::BindGroup>,
    _type: PhantomData<T>,
}

impl<T> TaroDataUploader for T
where
    T: TaroBytesBuilder,
{
    type UploadData = TaroBuffer<T>;

    fn new_upload(&self, hardware: &TaroHardware) -> Self::UploadData {
        TaroBuffer::<T>::new(self, hardware)
    }
}

impl<T> TaroData<T> for TaroBuffer<T>
where
    T: TaroBytesBuilder,
{
    fn write_new(&self, new_data: &T, hardware: &TaroHardware) {
        hardware
            .queue()
            .write_buffer(&self.buffer, 0, new_data.as_bytes());
    }
}

impl<T> TaroBuffer<T>
where
    T: TaroBytesBuilder,
{
    fn new(data: &T, hardware: &TaroHardware) -> Self {
        Self {
            _type: Default::default(),
            binding_cache: OnceMap::new(),
            buffer: hardware
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{} Buffer", type_name::<T>())),
                    contents: data.as_bytes(),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }),
        }
    }

    pub fn get_or_init_binding<Shader>(
        &self,
        shader: &Shader,
        layout: &wgpu::BindGroupLayout,
        hardware: &TaroHardware,
    ) -> &wgpu::BindGroup
    where
        Shader: TaroCoreShader,
    {
        let _unuse = shader;
        let shader_id = TypeId::of::<Shader>();
        self.binding_cache
            .get_or_init(&shader_id, || {
                hardware
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.buffer.as_entire_binding(),
                        }],
                        label: Some(&format!("{} BindGroup", type_name::<T>())),
                    })
            })
            .into_data()
    }
}

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
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn new(initial_value: &T, layout: &wgpu::BindGroupLayout, hardware: &TaroHardware) -> Self {
        let buffer = hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Buffer", type_name::<T>())),
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
                label: Some(&format!("{} BindGroup", type_name::<T>())),
            });

        Self {
            hardware_id: hardware.id().clone(),
            buffer,
            bind_group,
            _type: Default::default(),
        }
    }

    pub fn write(&self, new_data: &T, hardware: &TaroHardware) {
        if hardware.id() != &self.hardware_id {
            panic!(
                "Tried to set ShaderParameter<{:?}> with TaroHardware that does not match the original.",
                type_name::<T>()
            )
        }

        hardware
            .queue()
            .write_buffer(&self.buffer, 0, new_data.as_bytes());
    }
}
