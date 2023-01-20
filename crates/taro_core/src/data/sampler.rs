use std::num::NonZeroU8;

use crate::{BindingCompiler, Compiler, Taro};

#[derive(Clone)]
pub struct SamplerSettings {
    /// How to deal with out of bounds accesses in the u (i.e. x) direction
    pub address_mode_u: wgpu::AddressMode,
    /// How to deal with out of bounds accesses in the v (i.e. y) direction
    pub address_mode_v: wgpu::AddressMode,
    /// How to deal with out of bounds accesses in the w (i.e. z) direction
    pub address_mode_w: wgpu::AddressMode,
    /// How to filter the texture when it needs to be magnified (made larger)
    pub mag_filter: wgpu::FilterMode,
    /// How to filter the texture when it needs to be minified (made smaller)
    pub min_filter: wgpu::FilterMode,
    /// How to filter between mip map levels
    pub mipmap_filter: wgpu::FilterMode,
    /// Minimum level of detail (i.e. mip level) to use
    pub lod_min_clamp: f32,
    /// Maximum level of detail (i.e. mip level) to use
    pub lod_max_clamp: f32,
    /// If this is enabled, this is a comparison sampler using the given comparison function.
    pub compare: Option<wgpu::CompareFunction>,
    /// Valid values: 1, 2, 4, 8, and 16.
    pub anisotropy_clamp: Option<NonZeroU8>,
    /// Border color to use when address_mode is [`AddressMode::ClampToBorder`]
    pub border_color: Option<wgpu::SamplerBorderColor>,
}

impl Default for SamplerSettings {
    fn default() -> Self {
        Self {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: std::f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

pub trait SamplerBuilder: 'static {
    const BIND_TYPE: wgpu::SamplerBindingType;
}

pub struct Sampler {
    settings: SamplerSettings,
}

impl Default for Taro<Sampler> {
    fn default() -> Self {
        Sampler::new()
    }
}

impl Sampler {
    pub fn new() -> Taro<Self> {
        Self::from_settings(Default::default())
    }

    pub fn from_settings(settings: SamplerSettings) -> Taro<Self> {
        Taro::new(Self { settings })
    }

    pub fn settings(&self) -> &SamplerSettings {
        &self.settings
    }
}

impl Compiler for Sampler {
    type Compiled = wgpu::Sampler;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let settings = self.settings();
        let descriptor = wgpu::SamplerDescriptor {
            label: Some("Taro Sampler"),
            address_mode_u: settings.address_mode_u,
            address_mode_v: settings.address_mode_v,
            address_mode_w: settings.address_mode_w,
            mag_filter: settings.mag_filter,
            min_filter: settings.min_filter,
            mipmap_filter: settings.mipmap_filter,
            lod_max_clamp: settings.lod_max_clamp,
            lod_min_clamp: settings.lod_min_clamp,
            compare: settings.compare,
            anisotropy_clamp: settings.anisotropy_clamp,
            border_color: settings.border_color,
        };
        hardware.device().create_sampler(&descriptor)
    }
}

impl BindingCompiler for Taro<Sampler> {
    const LABEL: &'static str = "Taro Sampler Binding";
    const COUNT: Option<std::num::NonZeroU32> = None;
    const BIND_TYPE: wgpu::BindingType =
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering);

    fn compile_new_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {
        wgpu::BindingResource::Sampler(self.get_or_compile(hardware))
    }
}
