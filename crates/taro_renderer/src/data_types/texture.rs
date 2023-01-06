use std::{any::TypeId, rc::Rc};

use image::{DynamicImage, GenericImageView};
use once_map::OnceMap;
use wgpu::util::DeviceExt;

use crate::{shading::TaroCoreShader, HardwareId, TaroHardware};

pub struct UploadedTaroTexture {
    hardware_id: HardwareId,
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,

    binding_cache: OnceMap<TypeId, wgpu::BindGroup>,
}

pub trait ShaderTextureExt {
    fn get_or_init_texture_binding<'a>(
        &'a self,
        buffer: &'a UploadedTaroTexture,
        layout: &wgpu::BindGroupLayout,
        hardware: &TaroHardware,
    ) -> &wgpu::BindGroup;
}

impl<T> ShaderTextureExt for T
where
    T: TaroCoreShader,
{
    fn get_or_init_texture_binding<'a>(
        &'a self,
        buffer: &'a UploadedTaroTexture,
        layout: &wgpu::BindGroupLayout,
        hardware: &TaroHardware,
    ) -> &wgpu::BindGroup {
        if &buffer.hardware_id != hardware.id() {
            panic!(
                "Tried to get UploadedTaroTexture bind group using TaroHardware that does not match the original compiler."
            );
        }

        let typeid = TypeId::of::<Self>();
        buffer
            .binding_cache
            .get_or_init(&typeid, || {
                hardware
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(&buffer.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(&buffer.sampler),
                            },
                        ],
                        label: Some(&format!("UploadedTaroTexture BindGroup")),
                    })
            })
            .into_data()
    }
}

#[derive(Clone)]
pub struct TaroTexture {
    image: Rc<DynamicImage>,
    cache: OnceMap<HardwareId, UploadedTaroTexture>,
}

impl TaroTexture {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            image: Rc::new(image),
            cache: Default::default(),
        }
    }

    pub fn get_uploaded(&self, hardware: &TaroHardware) -> &UploadedTaroTexture {
        self.cache
            .get_or_init(hardware.id(), || {
                let dimensions = self.image.dimensions();
                let texture_size = wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                };

                let texture = hardware.device().create_texture_with_data(
                    hardware.queue(),
                    &wgpu::TextureDescriptor {
                        label: Some("Taro Texture"),
                        size: texture_size,
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    },
                    &self.image.to_rgba8(),
                );

                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                let sampler = hardware.device().create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });

                UploadedTaroTexture {
                    hardware_id: hardware.id().clone(),
                    _texture: texture,
                    view,
                    sampler,
                    binding_cache: Default::default(),
                }
            })
            .into_data()
    }
}
