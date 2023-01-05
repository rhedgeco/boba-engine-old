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

pub trait TaroCoreShader: 'static {
    type InitParameters;
    fn build_instance(init: &Self::InitParameters, hardware: &TaroHardware) -> Self;
}

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
    pub fn new(parameters: T::InitParameters) -> Self {
        Self {
            parameters,
            shader_cache: Default::default(),
        }
    }

    pub fn upload(&self, hardware: &TaroHardware) -> &T {
        self.shader_cache
            .get_or_init(hardware.id(), || {
                T::build_instance(&self.parameters, hardware)
            })
            .into_data()
    }
}

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
    fn get_or_init_binding<'a, Data>(
        &'a self,
        buffer: &'a TaroBuffer<Data>,
        layout: &wgpu::BindGroupLayout,
        hardware: &TaroHardware,
    ) -> &wgpu::BindGroup {
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
