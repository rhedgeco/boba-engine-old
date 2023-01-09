use std::{any::type_name, marker::PhantomData, sync::Arc};

use indexmap::IndexMap;
use once_map::OnceMap;
use wgpu::util::DeviceExt;

use crate::{HardwareId, TaroHardware};

pub trait TaroBindingBuilder: Clone {
    fn build_bind_type(&self) -> wgpu::BindingType;
    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource;
}

struct InnerBinding<T> {
    visibility: wgpu::ShaderStages,
    bind_type: wgpu::BindingType,
    binder: T,
}

pub struct BindGroupData {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

pub struct TaroBinding<T>
where
    T: TaroBindingBuilder,
{
    inner: Arc<InnerBinding<T>>,
    single_cache: OnceMap<HardwareId, BindGroupData>,
}

impl<T> TaroBinding<T>
where
    T: TaroBindingBuilder,
{
    pub fn new(binder: T, visibility: wgpu::ShaderStages) -> Self {
        let inner = InnerBinding {
            visibility,
            bind_type: binder.build_bind_type(),
            binder,
        };

        Self {
            inner: Arc::new(inner),
            single_cache: Default::default(),
        }
    }

    pub fn binding_data(&self) -> &T {
        &self.inner.binder
    }

    pub fn get_or_compile_single<'a>(&'a self, hardware: &TaroHardware) -> &BindGroupData {
        self.single_cache
            .get_or_init(hardware.id().clone(), || {
                let layout =
                    hardware
                        .device()
                        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            label: Some("TaroTextureViewBinding Bind Group Layout"),
                            entries: &[wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: self.inner.visibility,
                                ty: self.inner.bind_type,
                                count: None,
                            }],
                        });

                let bind_group = hardware
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("TaroTextureViewBinding Bind Group"),
                        layout: &layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.inner.binder.build_bind_resource(hardware),
                        }],
                    });

                BindGroupData { layout, bind_group }
            })
            .into_data()
    }
}

trait DynamicBindingBuilder {
    fn visibility(&self) -> wgpu::ShaderStages;
    fn build_bind_type(&self) -> wgpu::BindingType;
    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource;
}

impl<T> DynamicBindingBuilder for TaroBinding<T>
where
    T: TaroBindingBuilder,
{
    fn visibility(&self) -> wgpu::ShaderStages {
        self.inner.visibility
    }

    fn build_bind_type(&self) -> wgpu::BindingType {
        self.inner.binder.build_bind_type()
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        self.inner.binder.build_bind_resource(hardware)
    }
}

pub struct BindGroup {
    bindings: Arc<IndexMap<u32, Box<dyn DynamicBindingBuilder>>>,
    cache: OnceMap<HardwareId, BindGroupData>,
}

impl BindGroup {
    pub fn get_or_compile(&self, hardware: &TaroHardware) -> &BindGroupData {
        self.cache
            .get_or_init(hardware.id().clone(), || {
                let layout_entries: Vec<_> = self
                    .bindings
                    .iter()
                    .map(|(index, b)| wgpu::BindGroupLayoutEntry {
                        binding: *index,
                        visibility: b.visibility(),
                        ty: b.build_bind_type(),
                        count: None,
                    })
                    .collect();

                let bind_group_entries: Vec<_> = self
                    .bindings
                    .iter()
                    .map(|(index, b)| wgpu::BindGroupEntry {
                        binding: *index,
                        resource: b.build_bind_resource(hardware),
                    })
                    .collect();

                let layout =
                    hardware
                        .device()
                        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            label: Some("TaroBindGroup Layout"),
                            entries: &layout_entries,
                        });

                let bind_group = hardware
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("TaroBindGroup"),
                        layout: &layout,
                        entries: &bind_group_entries,
                    });

                BindGroupData { layout, bind_group }
            })
            .into_data()
    }
}

#[derive(Default)]
pub struct BindGroupBuilder {
    bindings: IndexMap<u32, Box<dyn DynamicBindingBuilder>>,
}

impl BindGroupBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_binding<T>(mut self, binding_index: u32, binding: TaroBinding<T>) -> Self
    where
        T: TaroBindingBuilder + 'static,
    {
        self.bindings.insert(binding_index, Box::new(binding));
        self
    }

    pub fn build(self) -> BindGroup {
        BindGroup {
            bindings: Arc::new(self.bindings),
            cache: Default::default(),
        }
    }
}

pub trait TaroBytesBuilder: Default {
    fn build_bytes(&self) -> &[u8];
}

pub struct UniformBuffer<T>
where
    T: TaroBytesBuilder,
{
    cache: OnceMap<HardwareId, wgpu::Buffer>,
    _type: PhantomData<T>,
}

impl<T> Default for UniformBuffer<T>
where
    T: TaroBytesBuilder,
{
    fn default() -> Self {
        Self {
            cache: Default::default(),
            _type: Default::default(),
        }
    }
}

impl<T> Clone for UniformBuffer<T>
where
    T: TaroBytesBuilder,
{
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            _type: self._type.clone(),
        }
    }
}

impl<T> TaroBindingBuilder for UniformBuffer<T>
where
    T: TaroBytesBuilder,
{
    fn build_bind_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        self.get_or_default(hardware).as_entire_binding()
    }
}

impl<T> UniformBuffer<T>
where
    T: TaroBytesBuilder,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write_buffer(&self, data: &T, hardware: &TaroHardware) -> &wgpu::Buffer {
        match self.cache.get_or_init(hardware.id().clone(), || {
            hardware
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Taro Buffer<{:?}>", type_name::<T>())),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    contents: data.build_bytes(),
                })
        }) {
            once_map::GetOrInitData::Init(buffer) => buffer,
            once_map::GetOrInitData::Get(buffer) => {
                hardware
                    .queue()
                    .write_buffer(&buffer, 0, data.build_bytes());
                buffer
            }
        }
    }

    pub fn get_or_default(&self, hardware: &TaroHardware) -> &wgpu::Buffer {
        self.cache
            .get_or_init(hardware.id().clone(), || {
                hardware
                    .device()
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Taro Buffer<{:?}>", type_name::<T>())),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        contents: T::default().build_bytes(),
                    })
            })
            .into_data()
    }
}
