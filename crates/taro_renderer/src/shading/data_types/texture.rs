use std::{
    num::{NonZeroU32, NonZeroU8},
    sync::Arc,
};

use image::{GenericImageView, ImageResult, RgbaImage};
use once_map::OnceMap;
use wgpu::util::DeviceExt;

use crate::{shading::TaroBindingBuilder, HardwareId, TaroHardware};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DynamicImageFormat {
    Srgb,
    Linear,
}

impl Into<wgpu::TextureFormat> for DynamicImageFormat {
    fn into(self) -> wgpu::TextureFormat {
        match self {
            Self::Linear => wgpu::TextureFormat::Rgba8Unorm,
            Self::Srgb => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }
}

pub trait TextureDimensionExt {
    fn into_view_dimension(self) -> wgpu::TextureViewDimension;
}

impl TextureDimensionExt for wgpu::TextureDimension {
    fn into_view_dimension(self) -> wgpu::TextureViewDimension {
        match self {
            wgpu::TextureDimension::D1 => wgpu::TextureViewDimension::D1,
            wgpu::TextureDimension::D2 => wgpu::TextureViewDimension::D2,
            wgpu::TextureDimension::D3 => wgpu::TextureViewDimension::D3,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TextureSettings {
    /// Mip count of texture. For a texture with no extra mips, this must be 1.
    pub mip_level_count: u32,
    /// Sample count of texture. If this is not 1, texture must have [`BindingType::Texture::multisampled`] set to true.
    pub sample_count: u32,
    /// Format of rhe image
    pub format: DynamicImageFormat,
}

struct InnerTexture {
    buffer: RgbaImage,
    settings: TextureSettings,
    size: wgpu::Extent3d,
}

#[derive(Clone)]
pub struct Texture2D {
    inner: Arc<InnerTexture>,
    cache: OnceMap<HardwareId, wgpu::Texture>,
}

impl Texture2D {
    /// Creates a new TaroTexture out of `image` using the default settings
    pub fn new(buffer: &[u8]) -> ImageResult<Self> {
        let settings = TextureSettings {
            mip_level_count: 1,
            sample_count: 1,
            format: DynamicImageFormat::Srgb,
        };

        Self::new_with_settings(buffer, settings)
    }

    /// Creates a new TaroTexture out of `image` using custom `settings`
    pub fn new_with_settings(buffer: &[u8], settings: TextureSettings) -> ImageResult<Self> {
        let image = image::load_from_memory(buffer)?;
        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let inner = InnerTexture {
            buffer: image.to_rgba8(),
            settings,
            size,
        };

        Ok(Self {
            inner: Arc::new(inner),
            cache: Default::default(),
        })
    }

    /// Gets the internal byte buffer used for this texture
    pub fn buffer(&self) -> &[u8] {
        &self.inner.buffer
    }

    /// Gets the internal `TextureSettings` used for this texture
    pub fn settings(&self) -> &TextureSettings {
        &self.inner.settings
    }

    /// Gets a GPU texture using the provided `hardware`
    ///
    /// If the texture has not been uploaded to the specified `hardware` yet. It will be done now.
    pub fn get_or_compile(&self, hardware: &TaroHardware) -> &wgpu::Texture {
        self.cache
            .get_or_init(hardware.id().clone(), || {
                let settings = self.settings();
                let descriptor = wgpu::TextureDescriptor {
                    label: Some("Taro Texture"),
                    size: self.inner.size,
                    mip_level_count: settings.mip_level_count,
                    sample_count: settings.sample_count,
                    dimension: wgpu::TextureDimension::D2,
                    format: settings.format.into(),
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                };

                hardware.device().create_texture_with_data(
                    hardware.queue(),
                    &descriptor,
                    &self.buffer(),
                )
            })
            .into_data()
    }
}

struct InnerTextureView {
    texture: Texture2D,
}

#[derive(Clone)]
pub struct Texture2DView {
    inner: Arc<InnerTextureView>,
    cache: OnceMap<HardwareId, wgpu::TextureView>,
}

impl TaroBindingBuilder for Texture2DView {
    fn build_bind_type(&self) -> wgpu::BindingType {
        let texture_settings = self.texture().settings();
        wgpu::BindingType::Texture {
            multisampled: texture_settings.sample_count > 1,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        }
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        wgpu::BindingResource::TextureView(self.get_or_compile(hardware))
    }
}

impl Texture2DView {
    /// Creates a new TaroTextureView out of a TaroTexture.
    ///
    /// This does its best to assume the default settings for this texture base on its descriptor.
    /// However, if you manually created the texture from raw bytes and a descriptor, not all assumptions may be covered.
    pub fn new(texture: Texture2D) -> Self {
        let inner = InnerTextureView { texture };

        Self {
            inner: Arc::new(inner),
            cache: Default::default(),
        }
    }

    /// Gets the internal texture used for this view
    pub fn texture(&self) -> &Texture2D {
        &self.inner.texture
    }

    /// Gets a GPU texture view using the provided `hardware`
    ///
    /// If the texture view has not been uploaded to the specified `hardware` yet. It will be done now.
    pub fn get_or_compile(&self, hardware: &TaroHardware) -> &wgpu::TextureView {
        self.cache
            .get_or_init(hardware.id().clone(), || {
                let settings = self.texture().settings();
                let descriptor = wgpu::TextureViewDescriptor {
                    label: Some("Taro Texture View"),
                    format: Some(settings.format.into()),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: NonZeroU32::new(settings.mip_level_count),
                    base_array_layer: 0,
                    array_layer_count: None,
                };

                self.inner
                    .texture
                    .get_or_compile(hardware)
                    .create_view(&descriptor)
            })
            .into_data()
    }
}

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
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: std::f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

struct InnerSampler {
    binding_type: wgpu::SamplerBindingType,
    settings: SamplerSettings,
}

#[derive(Clone)]
pub struct TaroSampler {
    inner: Arc<InnerSampler>,
    cache: OnceMap<HardwareId, wgpu::Sampler>,
}

impl TaroBindingBuilder for TaroSampler {
    fn build_bind_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Sampler(self.inner.binding_type)
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        wgpu::BindingResource::Sampler(self.get_or_compile(hardware))
    }
}

impl TaroSampler {
    /// Creates a new TaroSampler with default settings
    pub fn new() -> Self {
        Self::from_settings(SamplerSettings {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        })
    }

    /// Creates a new TaroSampler with provided `settings`
    pub fn from_settings(settings: SamplerSettings) -> Self {
        let binding_type = match &settings {
            d if d.compare.is_some() => wgpu::SamplerBindingType::Comparison,
            d if d.mag_filter == wgpu::FilterMode::Linear
                || d.min_filter == wgpu::FilterMode::Linear
                || d.mipmap_filter == wgpu::FilterMode::Linear =>
            {
                wgpu::SamplerBindingType::Filtering
            }
            _ => wgpu::SamplerBindingType::NonFiltering,
        };

        let inner = InnerSampler {
            binding_type,
            settings,
        };

        Self {
            inner: Arc::new(inner),
            cache: Default::default(),
        }
    }

    /// Gets the internal `SamplerSettings` used for this texture
    pub fn settings(&self) -> &SamplerSettings {
        &self.inner.settings
    }

    /// Gets a GPU sampler using the provided `hardware`
    ///
    /// If the sampler has not been uploaded to the specified `hardware` yet. It will be done now.
    pub fn get_or_compile(&self, hardware: &TaroHardware) -> &wgpu::Sampler {
        self.cache
            .get_or_init(hardware.id().clone(), || {
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
            })
            .into_data()
    }
}
