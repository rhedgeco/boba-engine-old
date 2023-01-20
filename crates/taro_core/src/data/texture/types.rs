use super::TextureBuilder;

pub struct Simple {}

impl TextureBuilder for Simple {
    const LABEL: &'static str = "Simple";
    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

    fn build_bytes(image: image::DynamicImage) -> Vec<u8> {
        image.into_rgba8().into_vec()
    }
}
