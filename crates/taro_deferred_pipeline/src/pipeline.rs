use taro_core::{
    data::{
        buffers::CameraMatrix,
        texture::{Bgra8Srgb, Depth, RgbaF32, Texture2D, Texture2DView},
        Buffer, Uniform,
    },
    rendering::{
        shaders::LitShader, RenderPipeline, RenderTexture, TaroMeshRenderer, TaroRenderPearls,
    },
    wgpu, Bind, BindGroup, BindGroupBuilder, Taro, TaroHardware,
};

use crate::shaders::DeferredShader;

pub struct DeferredPipeline {
    image_size: (u32, u32),
    depth: Taro<Bind<Texture2DView<Depth>>>,
    position: Taro<Bind<Texture2DView<RgbaF32>>>,
    normal: Taro<Bind<Texture2DView<RgbaF32>>>,
    albedo: Taro<Bind<Texture2DView<Bgra8Srgb>>>,
    specular: Taro<Bind<Texture2DView<RgbaF32>>>,
    gbuffer: Taro<BindGroup>,
}

impl DeferredPipeline {
    pub fn new() -> Self {
        let depth = Bind::new(Texture2DView::from_texture(Texture2D::empty(1, 1)));
        let position = Bind::new(Texture2DView::from_texture(Texture2D::empty(1, 1)));
        let normal = Bind::new(Texture2DView::from_texture(Texture2D::empty(1, 1)));
        let albedo = Bind::new(Texture2DView::from_texture(Texture2D::empty(1, 1)));
        let specular = Bind::new(Texture2DView::from_texture(Texture2D::empty(1, 1)));
        let gbuffer = BindGroupBuilder::new(0, position.clone())
            .insert(1, normal.clone())
            .insert(2, albedo.clone())
            .insert(3, specular.clone())
            .build();
        Self {
            image_size: (1, 1),
            depth,
            position,
            normal,
            albedo,
            specular,
            gbuffer,
        }
    }
}

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

        // resize all buffers if necessary
        if &self.image_size != texture.size() {
            let width = texture.size().0;
            let height = texture.size().1;
            self.depth = Bind::new(Texture2DView::from_texture(Texture2D::empty(width, height)));
            self.position = Bind::new(Texture2DView::from_texture(Texture2D::empty(width, height)));
            self.normal = Bind::new(Texture2DView::from_texture(Texture2D::empty(width, height)));
            self.albedo = Bind::new(Texture2DView::from_texture(Texture2D::empty(width, height)));
            self.specular = Bind::new(Texture2DView::from_texture(Texture2D::empty(width, height)));
            self.gbuffer = BindGroupBuilder::new(0, self.position.clone())
                .insert(1, self.normal.clone())
                .insert(2, self.albedo.clone())
                .insert(3, self.specular.clone())
                .build();
        }

        let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &self.depth.bind_data().get_or_compile(hardware),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        };

        let lit_renderers = pearls.collect::<TaroMeshRenderer<LitShader>>();

        // --- POSITION PASS ---
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Position Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: self.position.bind_data().get_or_compile(hardware),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(depth_stencil_attachment.clone()),
            });

            for renderer in lit_renderers.iter() {
                let model_matrix = renderer.update_and_get_model_matrix(hardware);
                renderer.shader.render_gbuffer_position(
                    &renderer.mesh,
                    camera_matrix,
                    model_matrix,
                    &mut pass,
                    hardware,
                )
            }
        }

        // --- NORMAL PASS ---
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Normal Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.normal.bind_data().get_or_compile(hardware),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(depth_stencil_attachment.clone()),
            });

            for renderer in lit_renderers.iter() {
                let model_matrix = renderer.get_model_matrix();
                renderer.shader.render_gbuffer_normal(
                    &renderer.mesh,
                    camera_matrix,
                    model_matrix,
                    &mut pass,
                    hardware,
                )
            }
        }

        // --- ALBEDO PASS ---
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Albedo Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.albedo.bind_data().get_or_compile(hardware),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(depth_stencil_attachment.clone()),
            });

            for renderer in lit_renderers.iter() {
                let model_matrix = renderer.get_model_matrix();
                renderer.shader.render_gbuffer_albedo(
                    &renderer.mesh,
                    camera_matrix,
                    model_matrix,
                    &mut pass,
                    hardware,
                )
            }
        }

        // --- SPECULAR PASS ---
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Specular Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.specular.bind_data().get_or_compile(hardware),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(depth_stencil_attachment.clone()),
            });

            for renderer in lit_renderers.iter() {
                let model_matrix = renderer.get_model_matrix();
                renderer.shader.render_gbuffer_specular(
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
            texture: &self.albedo.bind_data().texture().get_or_compile(hardware),
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

        let render_size = wgpu::Extent3d {
            width: texture.size().0,
            height: texture.size().1,
            depth_or_array_layers: 1,
        };

        encoder.copy_texture_to_texture(copy_src, copy_dst, render_size);

        // submit command encoder for rendering
        hardware.queue().submit(std::iter::once(encoder.finish()));
    }
}
