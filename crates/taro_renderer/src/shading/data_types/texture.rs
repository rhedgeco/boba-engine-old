use std::num::{NonZeroU32, NonZeroU8};

use image::{ImageResult, RgbaImage};
use wgpu::util::DeviceExt;

use crate::{
    shading::{Taro, TaroBindBuilder, TaroBuilder},
    TaroHardware,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ImageFormat {
    Srgb,
    Linear,
}

impl Into<wgpu::TextureFormat> for ImageFormat {
    fn into(self) -> wgpu::TextureFormat {
        match self {
            Self::Linear => wgpu::TextureFormat::Rgba8Unorm,
            Self::Srgb => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }
}

pub struct Texture2D {
    size: (u32, u32),
    image: RgbaImage,
    format: ImageFormat,
}

impl Texture2D {
    pub fn new(buffer: &[u8]) -> ImageResult<Taro<Self>> {
        Self::new_with_format(buffer, ImageFormat::Srgb)
    }

    pub fn new_with_format(buffer: &[u8], format: ImageFormat) -> ImageResult<Taro<Self>> {
        let image = image::load_from_memory(buffer)?;
        Ok(Taro::new(Self {
            size: (image.width(), image.height()),
            image: image.into_rgba8(),
            format,
        }))
    }

    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }
}

impl TaroBuilder for Texture2D {
    type Compiled = wgpu::Texture;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        let size = wgpu::Extent3d {
            width: self.size.0,
            height: self.size.1,
            depth_or_array_layers: 1,
        };

        let descriptor = wgpu::TextureDescriptor {
            label: Some("Taro Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format.into(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
        };

        hardware
            .device()
            .create_texture_with_data(hardware.queue(), &descriptor, &self.image)
    }
}

pub struct Texture2DView {
    texture: Taro<Texture2D>,
}

impl Texture2DView {
    pub fn new(buffer: &[u8]) -> ImageResult<Taro<Self>> {
        Self::new_with_format(buffer, ImageFormat::Srgb)
    }

    pub fn new_with_format(buffer: &[u8], format: ImageFormat) -> ImageResult<Taro<Self>> {
        let texture = Texture2D::new_with_format(buffer, format)?;
        Ok(Self::from_texture(texture))
    }

    pub fn from_texture(texture: Taro<Texture2D>) -> Taro<Self> {
        Taro::new(Self { texture })
    }

    pub fn texture(&self) -> &Taro<Texture2D> {
        &self.texture
    }
}

impl TaroBuilder for Texture2DView {
    type Compiled = wgpu::TextureView;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        let descriptor = wgpu::TextureViewDescriptor {
            label: Some("Taro Texture View"),
            format: Some(self.texture.format.into()),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: NonZeroU32::new(1),
            base_array_layer: 0,
            array_layer_count: None,
        };
        self.texture
            .get_or_compile(hardware)
            .create_view(&descriptor)
    }
}

impl TaroBindBuilder for Taro<Texture2DView> {
    fn build_bind_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        }
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        wgpu::BindingResource::TextureView(self.get_or_compile(hardware))
    }
}

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

pub struct Sampler {
    bind_type: wgpu::SamplerBindingType,
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
        let bind_type = match &settings {
            d if d.compare.is_some() => wgpu::SamplerBindingType::Comparison,
            d if d.mag_filter == wgpu::FilterMode::Linear
                || d.min_filter == wgpu::FilterMode::Linear
                || d.mipmap_filter == wgpu::FilterMode::Linear =>
            {
                wgpu::SamplerBindingType::Filtering
            }
            _ => wgpu::SamplerBindingType::NonFiltering,
        };

        Taro::new(Self {
            bind_type,
            settings,
        })
    }

    pub fn settings(&self) -> &SamplerSettings {
        &self.settings
    }
}

impl TaroBuilder for Sampler {
    type Compiled = wgpu::Sampler;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
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

impl TaroBindBuilder for Taro<Sampler> {
    fn build_bind_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Sampler(self.bind_type)
    }

    fn build_bind_resource(&self, hardware: &TaroHardware) -> wgpu::BindingResource {
        wgpu::BindingResource::Sampler(self.get_or_compile(hardware))
    }
}

pub struct DepthView {
    size: (u32, u32),
}

impl DepthView {
    pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(size: (u32, u32)) -> Taro<DepthView> {
        Taro::new(DepthView { size })
    }
}

impl TaroBuilder for DepthView {
    type Compiled = wgpu::TextureView;

    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled {
        let size = wgpu::Extent3d {
            width: self.size.0,
            height: self.size.1,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Taro Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };

        let texture = hardware.device().create_texture(&desc);

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
