use taro_core::{
    data::{buffers::CameraMatrix, Buffer, Uniform},
    rendering::{RenderPipeline, RenderTexture, TaroRenderPearls},
    wgpu, Bind, Taro, TaroHardware,
};

use crate::DeferredRenderer;

pub struct DeferredPipeline;

impl RenderPipeline for DeferredPipeline {
    fn render(
        &mut self,
        texture: &RenderTexture,
        pearls: &TaroRenderPearls,
        camera_matrix: &Taro<Bind<Buffer<Uniform<CameraMatrix>>>>,
        hardware: &TaroHardware,
    ) {
        let mut encoder =
            hardware
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Deferred Pipeline Command Encoder"),
                });

        let render_size = wgpu::Extent3d {
            width: texture.size().0,
            height: texture.size().1,
            depth_or_array_layers: 1,
        };

        let depth_texture = hardware.device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Taro Depth Texture"),
            size: render_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        let depth_view = depth_texture.create_view(&Default::default());

        let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        });

        let albedo_texture = hardware.device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Deferred Albedo Texture"),
            size: render_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        });

        let albedo_view = albedo_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Deferred Albedo Texture View"),
            format: Some(wgpu::TextureFormat::Bgra8UnormSrgb),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        // --- ALBEDO PASS ---
        {
            let renderers = pearls.collect::<DeferredRenderer>();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Unlit Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &albedo_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment,
            });

            for renderer in renderers.iter() {
                let model_matrix = renderer.get_updated_model_matrix(hardware);
                renderer.render_gbuffer_albedo(
                    &renderer.mesh,
                    camera_matrix,
                    model_matrix,
                    &mut pass,
                    hardware,
                )
            }
        }

        // --- COPY ALBEDO BACK INTO RENDER TEXTURE ---
        let copy_src = wgpu::ImageCopyTexture {
            texture: &albedo_texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
            aspect: wgpu::TextureAspect::All,
        };
        let copy_dst = wgpu::ImageCopyTexture {
            texture: &texture.texture().texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
            aspect: wgpu::TextureAspect::All,
        };
        encoder.copy_texture_to_texture(copy_src, copy_dst, render_size);

        // submit command encoder for rendering
        hardware.queue().submit(std::iter::once(encoder.finish()));
    }
}
