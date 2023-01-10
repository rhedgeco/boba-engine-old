use crate::{HardwareId, TaroHardware};
use indexmap::IndexMap;
use once_map::OnceMap;
use std::{ops::Deref, sync::Arc};

/// A convenience type to simplify the creation of a single binding.
pub type TaroBindSingle<T> = Taro<Bind<Taro<T>>>;

/// Base trait for items to be built into a Taro<T> object
pub trait TaroBuilder {
    type Compiled;
    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled;
}

/// The core data type for TaroRenderer.
///
/// It holds read only data to be compiled and used on the GPU
pub struct Taro<T: TaroBuilder> {
    inner: Arc<T>,
    cache: OnceMap<HardwareId, T::Compiled>,
}

impl<T: TaroBuilder> Clone for Taro<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl<T: TaroBuilder> Deref for Taro<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: TaroBuilder> Taro<T> {
    /// Creates a new Taro object containing `data`
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(data),
            cache: Default::default(),
        }
    }

    /// Gets the compiled form of the internal data.
    ///
    /// If the data has been compiled already, it will be retrieved from the cache.
    pub fn get_or_compile(&self, hardware: &TaroHardware) -> &T::Compiled {
        self.cache
            .get_or_init(hardware.id().clone(), || self.inner.compile(hardware))
            .into_data()
    }
}

/// Base trait for items to be built into a binding.
pub trait TaroBindBuilder: 'static {
    fn build_bind_type(&self) -> wgpu::BindingType;
    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource;
}

/// Compiled data for a bind group.
///
/// Contains the bind group itself, and the layout used to create it.
pub struct BindGroupData {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

/// Used to create and compile wgpu bind groups
pub struct Bind<T: TaroBindBuilder> {
    bind_type: wgpu::BindingType,
    visibility: wgpu::ShaderStages,
    data: T,
}

impl<T: TaroBindBuilder> Bind<T> {
    /// Creates a new bind with `visibility` using the provided `data`
    pub fn new(data: T, visibility: wgpu::ShaderStages) -> Taro<Self> {
        Taro::new(Self {
            bind_type: data.build_bind_type(),
            visibility,
            data,
        })
    }

    /// Gets the underlying data for the binding
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T: TaroBindBuilder> TaroBuilder for Bind<T> {
    type Compiled = BindGroupData;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("Taro Bind {}", std::any::type_name::<T>())),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: self.visibility,
                    ty: self.bind_type,
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
                    resource: self.data.build_bind_resource(hardware),
                }],
            });

        BindGroupData { layout, bind_group }
    }
}

trait DynamicBindBuilder {
    fn visibility(&self) -> wgpu::ShaderStages;
    fn build_bind_type(&self) -> wgpu::BindingType;
    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource;
}

impl<T: TaroBindBuilder> DynamicBindBuilder for Taro<Bind<T>> {
    fn visibility(&self) -> wgpu::ShaderStages {
        self.visibility
    }

    fn build_bind_type(&self) -> wgpu::BindingType {
        self.data.build_bind_type()
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        self.data.build_bind_resource(hardware)
    }
}

pub struct BindGroup {
    bindings: IndexMap<u32, Box<dyn DynamicBindBuilder>>,
}

impl TaroBuilder for BindGroup {
    type Compiled = BindGroupData;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
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

        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Taro BindGroup Layout"),
                entries: &layout_entries,
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Taro BindGroup"),
                layout: &layout,
                entries: &bind_group_entries,
            });

        BindGroupData { layout, bind_group }
    }
}

#[derive(Default)]
pub struct BindGroupBuilder {
    bindings: IndexMap<u32, Box<dyn DynamicBindBuilder>>,
}

impl BindGroupBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert<T>(mut self, binding_index: u32, binding: Taro<Bind<T>>) -> Self
    where
        T: TaroBindBuilder,
    {
        self.bindings.insert(binding_index, Box::new(binding));
        self
    }

    pub fn build(self) -> Taro<BindGroup> {
        Taro::new(BindGroup {
            bindings: self.bindings,
        })
    }
}
