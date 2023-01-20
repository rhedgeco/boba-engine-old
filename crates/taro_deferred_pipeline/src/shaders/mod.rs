mod unlit;

pub use unlit::*;

use taro_core::{
    data::{
        buffers::{CameraMatrix, TransformMatrix},
        Buffer, Mesh, Uniform,
    },
    wgpu, Bind, Taro, TaroHardware,
};

pub trait DeferredShader: 'static {
    fn render_gbuffer_albedo<'pass>(
        &'pass self,
        mesh: &'pass Taro<Mesh>,
        camera_matrix: &'pass Taro<Bind<Buffer<Uniform<CameraMatrix>>>>,
        model_matrix: &'pass Taro<Bind<Buffer<Uniform<TransformMatrix>>>>,
        pass: &mut wgpu::RenderPass<'pass>,
        hardware: &TaroHardware,
    );
}
