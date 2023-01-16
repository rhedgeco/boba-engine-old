use std::marker::PhantomData;

use indexmap::IndexMap;

use crate::{Compiler, Taro, TaroHardware};

/// Required trait to be built into a [`Bind`] object
pub trait BindingCompiler: 'static {
    const BIND_TYPE: wgpu::BindingType;
    fn manual_compile_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource;
}

/// The generic untyped form of a [`CompiledSingleBinding`]
pub struct GenericCompiledSingleBinding {
    pub entry: wgpu::BindGroupLayoutEntry,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

/// Binding data that has been uploaded to the GPU
pub struct CompiledSingleBinding<T> {
    generic: GenericCompiledSingleBinding,
    _type: PhantomData<T>,
}

impl<T> CompiledSingleBinding<T> {
    /// Converts a typed compiled binding into its internal generic form
    ///
    /// This may be used in case you dont want typing information about a bind group
    pub fn into_generic(self) -> GenericCompiledSingleBinding {
        self.generic
    }
}

/// Manages data that can be compiled into a bind group
pub struct Bind<T: BindingCompiler> {
    bind_data: T,
    visibility: wgpu::ShaderStages,
}

impl<T: BindingCompiler> Bind<T> {
    /// Creates a new [`Bind`] object wrapped in [`Taro`]
    pub fn new(bind_data: T, visibility: wgpu::ShaderStages) -> Taro<Bind<T>> {
        Taro::new(Bind {
            bind_data,
            visibility,
        })
    }

    /// Skips the creation of a bind object, and directly compiles the data.
    ///
    /// This is useful for compiling without the overhead of it being wrapped in a [`Taro`] struct.
    pub fn direct_manual_compile(
        bind_data: T,
        visibility: wgpu::ShaderStages,
        hardware: &TaroHardware,
    ) -> CompiledSingleBinding<T> {
        Bind {
            bind_data,
            visibility,
        }
        .manual_compile(hardware)
    }

    /// Gets the underlying bind data for this binding
    pub fn bind_data(&self) -> &T {
        &self.bind_data
    }
}

impl<T: BindingCompiler> Compiler for Bind<T> {
    type Compiled = CompiledSingleBinding<T>;

    fn manual_compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        let entry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: self.visibility,
            ty: T::BIND_TYPE,
            count: None,
        };

        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("Taro Bind {}", std::any::type_name::<T>())),
                entries: &[entry],
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("TaroTextureViewBinding Bind Group"),
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.bind_data.manual_compile_resource(hardware),
                }],
            });

        CompiledSingleBinding {
            generic: GenericCompiledSingleBinding {
                entry,
                layout,
                bind_group,
            },
            _type: PhantomData,
        }
    }
}

trait AnyBindingCompiler {
    fn visibility(&self) -> wgpu::ShaderStages;
    fn get_bind_type(&self) -> wgpu::BindingType;
    fn manual_compile_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource;
}

impl<T: BindingCompiler> AnyBindingCompiler for Taro<Bind<T>> {
    fn visibility(&self) -> wgpu::ShaderStages {
        self.visibility
    }

    fn get_bind_type(&self) -> wgpu::BindingType {
        T::BIND_TYPE
    }

    fn manual_compile_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        self.bind_data.manual_compile_resource(hardware)
    }
}

/// A group of [`Bind`] objects compiled into a single bind group
pub struct CompiledBindGroup {
    pub entries: Vec<wgpu::BindGroupLayoutEntry>,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

/// A collection of bindings compiled into a single bind group
///
/// It comes with no typing information by default,
/// as it could hold any arbitrary number of bindings within it.
pub struct BindGroup {
    bindings: IndexMap<u32, Box<dyn AnyBindingCompiler>>,
}

impl Compiler for BindGroup {
    type Compiled = CompiledBindGroup;

    fn manual_compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        let entries: Vec<_> = self
            .bindings
            .iter()
            .map(|(index, b)| wgpu::BindGroupLayoutEntry {
                binding: *index,
                visibility: b.visibility(),
                ty: b.get_bind_type(),
                count: None,
            })
            .collect();

        let bind_group_entries: Vec<_> = self
            .bindings
            .iter()
            .map(|(index, b)| wgpu::BindGroupEntry {
                binding: *index,
                resource: b.manual_compile_resource(hardware),
            })
            .collect();

        let layout = hardware
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Taro BindGroup Layout"),
                entries: &entries,
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Taro BindGroup"),
                layout: &layout,
                entries: &bind_group_entries,
            });

        CompiledBindGroup {
            entries,
            layout,
            bind_group,
        }
    }
}

/// A builder used to create new [`BindGroup`] objects
#[derive(Default)]
pub struct BindGroupBuilder {
    bindings: IndexMap<u32, Box<dyn AnyBindingCompiler>>,
}

impl BindGroupBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a binding to be tracked by the bind group
    pub fn insert<T>(mut self, binding_index: u32, binding: Taro<Bind<T>>) -> Self
    where
        T: BindingCompiler,
    {
        self.bindings.insert(binding_index, Box::new(binding));
        self
    }

    /// Consumes the builder and creates a new [`BindGroup`]
    pub fn build(self) -> Taro<BindGroup> {
        Taro::new(BindGroup {
            bindings: self.bindings,
        })
    }
}
