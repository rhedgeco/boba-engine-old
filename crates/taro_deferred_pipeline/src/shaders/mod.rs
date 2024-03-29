mod lit;

pub use lit::*;

use taro_core::{
    data::{
        buffers::{CameraMatrix, TransformMatrix},
        Mesh, UniformBinding,
    },
    wgpu, Taro, TaroHardware,
};

pub trait DeferredShader: 'static {
    fn render_gbuffer_position<'pass>(
        &'pass self,
        mesh: &'pass Taro<Mesh>,
        camera_matrix: &'pass Taro<UniformBinding<CameraMatrix>>,
        model_matrix: &'pass Taro<UniformBinding<TransformMatrix>>,
        pass: &mut wgpu::RenderPass<'pass>,
        hardware: &TaroHardware,
    );

    fn render_gbuffer_normal<'pass>(
        &'pass self,
        mesh: &'pass Taro<Mesh>,
        camera_matrix: &'pass Taro<UniformBinding<CameraMatrix>>,
        model_matrix: &'pass Taro<UniformBinding<TransformMatrix>>,
        pass: &mut wgpu::RenderPass<'pass>,
        hardware: &TaroHardware,
    );

    fn render_gbuffer_albedo<'pass>(
        &'pass self,
        mesh: &'pass Taro<Mesh>,
        camera_matrix: &'pass Taro<UniformBinding<CameraMatrix>>,
        model_matrix: &'pass Taro<UniformBinding<TransformMatrix>>,
        pass: &mut wgpu::RenderPass<'pass>,
        hardware: &TaroHardware,
    );

    fn render_gbuffer_specular<'pass>(
        &'pass self,
        mesh: &'pass Taro<Mesh>,
        camera_matrix: &'pass Taro<UniformBinding<CameraMatrix>>,
        model_matrix: &'pass Taro<UniformBinding<TransformMatrix>>,
        pass: &mut wgpu::RenderPass<'pass>,
        hardware: &TaroHardware,
    );
}
