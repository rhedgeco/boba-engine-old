use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::TaroRenderer;

use super::TaroUploader;

pub struct TaroShader<'a> {
    module: Option<ShaderModule>,
    descriptor: Option<ShaderModuleDescriptor<'a>>,
}

impl<'a> TaroUploader for TaroShader<'a> {
    type UploadedData = ShaderModule;

    fn get_data(&self) -> &Option<Self::UploadedData> {
        &self.module
    }

    fn upload(&mut self, renderer: &TaroRenderer) {
        if self.module.is_some() {
            return;
        }

        let descriptor = std::mem::replace(&mut self.descriptor, None)
            .expect("Shader descriptor should be Some at this point");
        self.module = Some(renderer.device().create_shader_module(descriptor));
    }
}

impl<'a> TaroShader<'a> {
    pub fn from_str(label: Option<&'a str>, shader_code: &'a str) -> Self {
        let descriptor = ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(shader_code.into()),
        };

        TaroShader::<'a> {
            module: None,
            descriptor: Some(descriptor),
        }
    }
}
