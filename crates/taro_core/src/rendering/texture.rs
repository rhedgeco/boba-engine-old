pub struct RenderTexture {
    size: (u32, u32),
    texture: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
}

impl RenderTexture {
    pub fn new(size: (u32, u32), texture: wgpu::SurfaceTexture) -> Self {
        Self {
            view: texture.texture.create_view(&Default::default()),
            texture,
            size,
        }
    }

    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }

    pub fn texture(&self) -> &wgpu::SurfaceTexture {
        &self.texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn present(self) {
        self.texture.present()
    }
}
