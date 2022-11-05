use image::{DynamicImage, ImageError, RgbaImage};
use wgpu::{Extent3d, Texture, TextureDescriptor, TextureView};

use crate::TaroRenderer;

use super::TaroCompiler;

pub struct CompiledTaroTexture {
    pub texture: Texture,
    pub view: TextureView,
}

pub struct TaroTexture<'a> {
    rgba: RgbaImage,
    descriptor: TextureDescriptor<'a>,
    compiled: Option<CompiledTaroTexture>,
}

impl<'a> TaroCompiler for TaroTexture<'a> {
    type CompiledData = CompiledTaroTexture;

    fn get_data(&self) -> &Option<Self::CompiledData> {
        &self.compiled
    }

    fn compile(&mut self, renderer: &TaroRenderer) {
        if self.compiled.is_none() {
            let texture = renderer.device().create_texture(&self.descriptor);
            let size = self.descriptor.size;

            renderer.queue().write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                &self.rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * size.width),
                    rows_per_image: std::num::NonZeroU32::new(size.height),
                },
                size,
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.compiled = Some(CompiledTaroTexture { texture, view });
        }
    }
}

impl<'a> TaroTexture<'a> {
    pub fn from_bytes(label: Option<&'a str>, bytes: &[u8]) -> Result<Self, ImageError> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self::from_image(label, &image))
    }

    pub fn from_image(label: Option<&'a str>, image: &DynamicImage) -> Self {
        let rgba = image.to_rgba8();
        let dimensions = rgba.dimensions();
        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let descriptor = TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };

        Self {
            rgba,
            descriptor,
            compiled: None,
        }
    }
}
