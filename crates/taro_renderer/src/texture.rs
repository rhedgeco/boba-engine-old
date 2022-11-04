use image::{GenericImageView, ImageBuffer, Rgba};
use wgpu::{BindGroup, BindGroupLayout};

use crate::TaroRenderer;

pub struct TaroTexture {
    rgba8: ImageBuffer<Rgba<u8>, Vec<u8>>,
    size: wgpu::Extent3d,
    bind_group: Option<BindGroup>,
    bind_layout: Option<BindGroupLayout>,
}

impl TaroTexture {
    pub fn new(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        let rgba8 = image.to_rgba8();
        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        Self {
            rgba8,
            size,
            bind_group: None,
            bind_layout: None,
        }
    }

    pub fn bind_group(&self) -> &Option<BindGroup> {
        &self.bind_group
    }

    pub fn bind_layout(&self) -> &Option<BindGroupLayout> {
        &self.bind_layout
    }

    pub fn upload(&mut self, renderer: &TaroRenderer) {
        let texture = renderer.device().create_texture(&wgpu::TextureDescriptor {
            size: self.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture"),
        });

        renderer.queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.rgba8,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = renderer.device().create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        self.bind_layout = Some(renderer.device().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
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
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            },
        ));

        self.bind_group = Some(
            renderer
                .device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self
                        .bind_layout
                        .as_ref()
                        .expect("bind group layout should not be none"),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&texture_sampler),
                        },
                    ],
                    label: Some("texture_bind_group"),
                }),
        );
    }
}
