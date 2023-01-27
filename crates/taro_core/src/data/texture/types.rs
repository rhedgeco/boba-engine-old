use super::TextureBuilder;

pub struct Rgba8Srgb;

impl TextureBuilder for Rgba8Srgb {
    const LABEL: &'static str = "Rgba8Srgb";
    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

    fn build_bytes(image: image::DynamicImage) -> Vec<u8> {
        image.into_rgba8().into_vec()
    }
}

pub struct Bgra8Srgb;

impl TextureBuilder for Bgra8Srgb {
    const LABEL: &'static str = "Bgra8Srgb";
    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

    fn build_bytes(image: image::DynamicImage) -> Vec<u8> {
        image.into_rgba8().into_vec()
    }
}

pub struct RgbaF32;

impl TextureBuilder for RgbaF32 {
    const LABEL: &'static str = "RgbaF32";
    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;

    fn build_bytes(image: image::DynamicImage) -> Vec<u8> {
        bytemuck::cast_vec(image.into_rgba32f().into_vec())
    }
}

pub struct Depth;

impl TextureBuilder for Depth {
    const LABEL: &'static str = "Depth";
    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    fn build_bytes(image: image::DynamicImage) -> Vec<u8> {
        image.into_bytes()
    }
}
