use std::{marker::PhantomData, num::NonZeroU32};

use crate::{Compiler, Taro};

/// The generic untyped binding
pub struct CompiledBindGroup {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

/// Binding data that has been uploaded to the GPU
pub struct CompiledBind<T> {
    generic: CompiledBindGroup,
    _type: PhantomData<T>,
}

impl<T> CompiledBind<T> {
    /// Converts a typed compiled binding into its internal generic form
    ///
    /// This may be used in case you dont want typing information about a bind group
    pub fn into_generic(self) -> CompiledBindGroup {
        self.generic
    }
}

/// Represents all possible [`wgpu::ShaderStages`] combined
pub const ALL_SHADER_STAGES: wgpu::ShaderStages = wgpu::ShaderStages::from_bits_truncate(
    wgpu::ShaderStages::VERTEX_FRAGMENT.bits() | wgpu::ShaderStages::COMPUTE.bits(),
);

/// This must be implemented to be built into a [`Bind`] object
pub trait BindingCompiler: 'static {
    const LABEL: &'static str;
    const COUNT: Option<NonZeroU32>;
    const BIND_TYPE: wgpu::BindingType;
    fn compile_new_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource;
}

/// A struct that represents a binding to some kind of GPU data
pub struct Bind<T: Compiler>
where
    Taro<T>: BindingCompiler,
{
    data: Taro<T>,
    visibility: wgpu::ShaderStages,
}

impl<T: Compiler> Bind<T>
where
    Taro<T>: BindingCompiler,
{
    /// Creates a new binding to `data`
    pub fn new(data: Taro<T>) -> Taro<Self> {
        Self::new_with_visibility(data, ALL_SHADER_STAGES)
    }

    /// Creates a new binding to `data` with more restricted `visibility`
    pub fn new_with_visibility(data: Taro<T>, visibility: wgpu::ShaderStages) -> Taro<Self> {
        Taro::new(Self { data, visibility })
    }

    /// Gets the [`Taro`] object this is bound to
    pub fn get_bind_data(&self) -> &Taro<T> {
        &self.data
    }
}

impl<T: Compiler> Compiler for Bind<T>
where
    Taro<T>: BindingCompiler,
{
    type Compiled = CompiledBind<T>;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let label = <Taro<T> as BindingCompiler>::LABEL;

        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("{label} Bind Layout")),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: self.visibility,
                    ty: <Taro<T> as BindingCompiler>::BIND_TYPE,
                    count: <Taro<T> as BindingCompiler>::COUNT,
                }],
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                label: Some(&format!("{label} Bind")),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.data.compile_new_resource(hardware),
                }],
            });

        CompiledBind {
            _type: PhantomData,
            generic: CompiledBindGroup { layout, bind_group },
        }
    }
}

trait AnyBind {
    fn count(&self) -> Option<NonZeroU32>;
    fn bind_type(&self) -> wgpu::BindingType;
    fn visibility(&self) -> wgpu::ShaderStages;
    fn compile_new_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource;
}

impl<T: Compiler> AnyBind for Bind<T>
where
    Taro<T>: BindingCompiler,
{
    fn count(&self) -> Option<NonZeroU32> {
        <Taro<T> as BindingCompiler>::COUNT
    }

    fn bind_type(&self) -> wgpu::BindingType {
        <Taro<T> as BindingCompiler>::BIND_TYPE
    }

    fn visibility(&self) -> wgpu::ShaderStages {
        self.visibility
    }

    fn compile_new_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {
        self.data.compile_new_resource(hardware)
    }
}

/// A struct that represents a collection of [`Bind`] objects
pub struct BindGroup {
    bindings: Vec<Box<dyn AnyBind>>,
}

impl Compiler for BindGroup {
    type Compiled = CompiledBindGroup;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let layout_entries = self
            .bindings
            .iter()
            .map(|f| wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: f.visibility(),
                ty: f.bind_type(),
                count: f.count(),
            })
            .collect::<Vec<_>>();

        let bind_entries = self
            .bindings
            .iter()
            .map(|f| wgpu::BindGroupEntry {
                binding: 0,
                resource: f.compile_new_resource(hardware),
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

        CompiledBindGroup { layout, bind_group }
    }
}

/// A builder to create a new [`BindGroup`] object
pub struct BindGroupBuilder {
    bindings: Vec<Box<dyn AnyBind>>,
}

impl BindGroupBuilder {
    /// Creates a new builder and adds `bind` to it
    pub fn new<T: Compiler>(bind: Bind<T>) -> Self
    where
        Taro<T>: BindingCompiler,
    {
        let mut bindings: Vec<Box<dyn AnyBind>> = Vec::new();
        bindings.push(Box::new(bind));
        Self { bindings }
    }

    /// Adds another binding to the group
    pub fn add<T: Compiler>(mut self, bind: Bind<T>) -> Self
    where
        Taro<T>: BindingCompiler,
    {
        self.bindings.push(Box::new(bind));
        self
    }

    /// Consumes the builder and produces a [`BindGroup`]
    pub fn build(self) -> BindGroup {
        BindGroup {
            bindings: self.bindings,
        }
    }
}
