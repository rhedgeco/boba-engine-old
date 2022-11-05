use image::{DynamicImage, ImageError, RgbaImage};
use wgpu::{Extent3d, Texture, TextureDescriptor, TextureView};

use crate::TaroRenderer;

use super::TaroUploader;

pub struct TaroTexture<'a> {
    rgba: RgbaImage,
    descriptor: TextureDescriptor<'a>,
    texture: Option<(Texture, TextureView)>,
}

impl<'a> TaroUploader for TaroTexture<'a> {
    type UploadedData = (Texture, TextureView);

    fn get_data(&self) -> &Option<Self::UploadedData> {
        &self.texture
    }

    fn upload(&mut self, renderer: &TaroRenderer) {
        if self.texture.is_none() {
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
            self.texture = Some((texture, view));
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
            size: size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };

        Self {
            rgba,
            descriptor,
            texture: None,
        }
    }
}
