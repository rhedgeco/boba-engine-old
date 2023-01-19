// use crate::BindingCompiler;

// pub trait TexType: 'static {
//     const SAMPLE_TYPE: wgpu::TextureSampleType;
//     const DIMENSION: wgpu::TextureViewDimension;
//     const MULTISAMPLED: bool;
// }

// pub struct Texture<T: TexType> {
//     texture: wgpu::Texture,
//     view: wgpu::TextureView,
//     _type: T,
// }

// impl<T: TexType> BindingCompiler for Texture<T> {
//     const BIND_TYPE: wgpu::BindingType = wgpu::BindingType::Texture {
//         sample_type: T::SAMPLE_TYPE,
//         view_dimension: T::DIMENSION,
//         multisampled: T::MULTISAMPLED,
//     };

//     fn manual_compile_resource(&self, hardware: &crate::TaroHardware) -> wgpu::BindingResource {}
// }
