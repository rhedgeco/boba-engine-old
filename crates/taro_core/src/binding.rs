use std::{marker::PhantomData, num::NonZeroU32};

use indexmap::{indexmap, IndexMap};

use crate::{Compiler, Taro};

/// Compiled untyped binding data
pub struct CompiledBind {
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl CompiledBind {
    /// gets the underlying layout for the binding
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    /// gets the underlying bind group
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

/// Compiled binding data
pub struct CompiledTypedBind<T> {
    inner: CompiledBind,
    _type: PhantomData<T>,
}

impl<T> CompiledTypedBind<T> {
    pub fn to_untyped(self) -> CompiledBind {
        self.inner
    }

    /// gets the underlying layout for the binding
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.inner.layout()
    }

    /// gets the underlying bind group
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.inner.bind_group()
    }
}

/// Settings for a [`Bind`] object
pub struct BindSettings {
    pub label: &'static str,
    pub count: Option<NonZeroU32>,
}

impl BindSettings {
    /// Creates a new bind settings to be used in a `const` context
    pub const fn new(label: &'static str, count: Option<NonZeroU32>) -> Self {
        Self { label, count }
    }
}

/// Necessary to be built into a [`Bind`] object
pub trait BindCompiler {
    const SETTINGS: BindSettings;
    fn bind_type(&self) -> wgpu::BindingType;
    fn compile_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource;
}

/// Creates GPU bindings to [`Taro`] data
pub struct Bind<T>
where
    T: Compiler,
{
    bind_data: Taro<T>,
    visibility: wgpu::ShaderStages,
}

impl<T> Bind<T>
where
    T: Compiler,
    Taro<T>: BindCompiler,
{
    pub fn new(bind_data: Taro<T>) -> Taro<Self> {
        Taro::new(Self {
            bind_data,
            visibility: wgpu::ShaderStages::all(),
        })
    }

    pub fn bind_data(&self) -> &Taro<T> {
        &self.bind_data
    }
}

impl<T> Compiler for Bind<T>
where
    T: Compiler,
    Taro<T>: BindCompiler,
{
    type Compiled = CompiledTypedBind<T>;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let label = <Taro<T> as BindCompiler>::SETTINGS.label;

        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("{label} Bind Layout")),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: self.visibility,
                    ty: self.bind_data.bind_type(),
                    count: <Taro<T> as BindCompiler>::SETTINGS.count,
                }],
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                label: Some(&format!("{label} Binding")),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.bind_data.compile_resource(hardware),
                }],
            });

        let inner = CompiledBind { layout, bind_group };

        CompiledTypedBind {
            inner,
            _type: PhantomData,
        }
    }
}

trait SealedBindCompiler {
    fn settings(&self) -> BindSettings;
    fn bind_type(&self) -> wgpu::BindingType;
    fn visibility(&self) -> &wgpu::ShaderStages;
    fn compile_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource;
}

impl<T> SealedBindCompiler for Taro<Bind<T>>
where
    T: Compiler,
    Taro<T>: BindCompiler,
{
    fn settings(&self) -> BindSettings {
        <Taro<T> as BindCompiler>::SETTINGS
    }

    fn bind_type(&self) -> wgpu::BindingType {
        self.bind_data.bind_type()
    }

    fn visibility(&self) -> &wgpu::ShaderStages {
        &self.visibility
    }

    fn compile_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {
        self.bind_data.compile_resource(hardware)
    }
}

/// A struct that represents a collection of [`Bind`] objects
pub struct BindGroup {
    bindings: IndexMap<u32, Box<dyn SealedBindCompiler>>,
}

impl Compiler for BindGroup {
    type Compiled = CompiledBind;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let layout_entries = self
            .bindings
            .iter()
            .map(|(index, b)| wgpu::BindGroupLayoutEntry {
                binding: *index,
                visibility: *b.visibility(),
                ty: b.bind_type(),
                count: b.settings().count,
            })
            .collect::<Vec<_>>();

        let bind_entries = self
            .bindings
            .iter()
            .map(|(index, b)| wgpu::BindGroupEntry {
                binding: *index,
                resource: b.compile_resource(hardware),
            })
            .collect::<Vec<_>>();

        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("BindGroup Layout"),
                entries: &layout_entries,
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                label: Some("BindGroup"),
                entries: &bind_entries,
            });

        CompiledBind { layout, bind_group }
    }
}

/// A builder to create a new [`BindGroup`] object
pub struct BindGroupBuilder {
    bindings: IndexMap<u32, Box<dyn SealedBindCompiler>>,
}

impl BindGroupBuilder {
    /// Creates a new builder and adds `bind` to it
    pub fn new<T: Compiler>(index: u32, bind: Taro<Bind<T>>) -> Self
    where
        Taro<T>: BindCompiler,
    {
        let anybind: Box<dyn SealedBindCompiler> = Box::new(bind);
        Self {
            bindings: indexmap! {index => anybind},
        }
    }

    /// Inserts another binding to the group
    pub fn insert<T: Compiler>(mut self, index: u32, bind: Taro<Bind<T>>) -> Self
    where
        Taro<T>: BindCompiler,
    {
        self.bindings.insert(index, Box::new(bind));
        self
    }

    /// Consumes the builder and produces a [`BindGroup`]
    pub fn build(self) -> Taro<BindGroup> {
        Taro::new(BindGroup {
            bindings: self.bindings,
        })
    }
}
