use image::{DynamicImage, ImageError, RgbaImage};
use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, Extent3d, TextureDescriptor};

use crate::TaroHardware;

pub struct CompiledTaroTexture {
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

pub struct TaroTexture {
    label: Box<str>,
    rgba: RgbaImage,
    size: Extent3d,
    compiled: Option<CompiledTaroTexture>,
}

impl TaroTexture {
    pub fn from_bytes(label: &str, bytes: &[u8]) -> Result<Self, ImageError> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self::from_image(label, &image))
    }

    pub fn from_image(label: &str, image: &DynamicImage) -> Self {
        let rgba = image.to_rgba8();
        let dimensions = rgba.dimensions();
        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        Self {
            label: Box::<str>::from(label),
            rgba,
            size,
            compiled: None,
        }
    }

    pub fn compile(&mut self, hardware: &TaroHardware) -> &CompiledTaroTexture {
        if self.compiled.is_some() {
            return self.compiled.as_ref().unwrap();
        }

        let descriptor = TextureDescriptor {
            label: Some(&self.label),
            size: self.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };

        let texture = hardware.device.create_texture(&descriptor);

        hardware.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &self.rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Render Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let sampler = hardware.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Render Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

        self.compiled = Some(CompiledTaroTexture {
            bind_group,
            bind_group_layout,
        });

        self.compiled.as_ref().unwrap()
    }
}
