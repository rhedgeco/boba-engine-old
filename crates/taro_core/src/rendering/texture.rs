pub struct RenderTexture {
    pub size: (u32, u32),
    pub texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
}
