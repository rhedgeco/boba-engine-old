use log::error;
use taro_renderer::{
    renderers::TaroMeshRenderer,
    shading::{RenderMeshData, TaroMeshShader, TaroShaderCore},
    wgpu, TaroRenderPhase,
};

use crate::shaders::UnlitShader;

pub struct UnlitRenderPhase;

impl TaroRenderPhase for UnlitRenderPhase {
    fn render(
        &mut self,
        view: &taro_renderer::wgpu::TextureView,
        camera: &taro_renderer::wgpu::BindGroup,
        encoder: &mut taro_renderer::wgpu::CommandEncoder,
        pearls: &taro_renderer::RenderPearls,
        resources: &taro_renderer::RenderResources,
    ) {
        let mut shader = UnlitShader::default();
        shader.prepare(resources);

        let mut meshes = pearls.collect::<TaroMeshRenderer>();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            let binding = match mesh.mesh_binding(resources) {
                Ok(b) => b,
                Err(e) => {
                    error!("Cannot render a mesh. There was an error when getting the mesh bindings. Error: {e}");
                    continue;
                }
            };

            let data = RenderMeshData {
                mesh: binding.mesh,
                camera_bind_group: camera,
                model_bind_group: binding.matrix,
            };

            shader.render_mesh(&mut pass, &data);
        }
    }
}
