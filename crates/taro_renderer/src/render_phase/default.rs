use wgpu::{BindGroup, CommandEncoder, TextureView};

use crate::{renderers::TaroMeshRenderer, RenderPearls, RenderResources, TaroRenderPhase};

pub struct DefaultTaroPhase;

impl TaroRenderPhase for DefaultTaroPhase {
    fn render(
        &mut self,
        view: &TextureView,
        camera: &BindGroup,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
        resources: &RenderResources,
    ) {
        let mut meshes = pearls.collect::<TaroMeshRenderer>();
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        for mesh in meshes.iter_mut() {
            mesh.render_mesh(camera, &mut render_pass, resources);
            // let pipeline = mesh.pipeline().as_ref().unwrap();
            // let mesh_buffers = mesh.mesh().get_data().as_ref().unwrap();
            // let texture = mesh.texture().get_data().as_ref().unwrap();

            // render_pass.set_pipeline(&pipeline.render_pipeline);
            // render_pass.set_bind_group(0, &texture.bind_group, &[]);
            // render_pass.set_bind_group(1, camera, &[]);
            // render_pass.set_vertex_buffer(0, mesh_buffers.vertex_buffer.raw_buffer().slice(..));
            // render_pass.set_index_buffer(
            //     mesh_buffers.index_buffer.raw_buffer().slice(..),
            //     wgpu::IndexFormat::Uint16,
            // );
            // render_pass.draw_indexed(0..mesh_buffers.index_buffer.buffer_length(), 0, 0..1);
        }
    }
}
