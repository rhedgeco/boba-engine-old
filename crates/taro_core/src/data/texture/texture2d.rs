use std::marker::PhantomData;

use image::{DynamicImage, ImageResult};
use wgpu::util::DeviceExt;

use crate::{BindCompiler, BindSettings, Compiler, Taro};

pub trait TextureBuilder: 'static {
    const LABEL: &'static str;
    const FORMAT: wgpu::TextureFormat;
    fn build_bytes(image: DynamicImage) -> Vec<u8>;
}

pub struct Texture2D<T: TextureBuilder> {
    size: (u32, u32),
    bytes: Option<Vec<u8>>,
    usage: wgpu::TextureUsages,
    _type: PhantomData<T>,
}

impl<T: TextureBuilder> Texture2D<T> {
    pub fn empty(width: u32, height: u32) -> Taro<Self> {
        Taro::new(Self {
            size: (width, height),
            bytes: None,
            _type: PhantomData,
            usage: wgpu::TextureUsages::empty(),
        })
    }

    pub fn from_bytes(buffer: &[u8]) -> ImageResult<Taro<Self>> {
        let image = image::load_from_memory(buffer)?;
        Ok(Taro::new(Self {
            size: (image.width(), image.height()),
            bytes: Some(T::build_bytes(image)),
            _type: PhantomData,
            usage: wgpu::TextureUsages::empty(),
        }))
    }
}

impl<T: TextureBuilder> Compiler for Texture2D<T> {
    type Compiled = wgpu::Texture;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let label = format!("{} Texture", T::LABEL);

        let usage = self.usage
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT;

        let size = wgpu::Extent3d {
            width: self.size.0,
            height: self.size.1,
            depth_or_array_layers: 1,
        };

        let descriptor = wgpu::TextureDescriptor {
            label: Some(&label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: T::FORMAT,
            usage,
        };

        match &self.bytes {
            None => hardware.device().create_texture(&descriptor),
            Some(bytes) => {
                hardware
                    .device()
                    .create_texture_with_data(hardware.queue(), &descriptor, bytes)
            }
        }
    }
}

pub struct Texture2DView<T: TextureBuilder> {
    texture: Taro<Texture2D<T>>,
}

impl<T: TextureBuilder> Texture2DView<T> {
    pub fn from_texture(texture: Taro<Texture2D<T>>) -> Taro<Self> {
        Taro::new(Self { texture })
    }

    pub fn texture(&self) -> &Taro<Texture2D<T>> {
        &self.texture
    }
}

impl<T: TextureBuilder> Compiler for Texture2DView<T> {
    type Compiled = wgpu::TextureView;

    fn new_taro_compile(&self, hardware: &crate::TaroHardware) -> Self::Compiled {
        let label = format!("{} Texture View", T::LABEL);

        let descriptor = wgpu::TextureViewDescriptor {
            label: Some(&label),
            format: Some(T::FORMAT),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        };

        self.texture
            .get_or_compile(hardware)
            .create_view(&descriptor)
    }
}

impl<T: TextureBuilder> BindCompiler for Taro<Texture2DView<T>> {
    // const LABEL: &'static str = "Texture2D View";
    // const COUNT: Option<std::num::NonZeroU32> = None;
    // const BIND_TYPE: wgpu::BindingType = wgpu::BindingType::Texture {
    //     sample_type: wgpu::TextureSampleType::Float { filterable: true },
    //     view_dimension: wgpu::TextureViewDimension::D2,
    //     multisampled: false,
    // };
    const SETTINGS: BindSettings = BindSettings::new("Texture2D View", None);

    fn bind_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    }

    fn compile_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {
        wgpu::BindingResource::TextureView(self.get_or_compile(hardware))
    }
}
