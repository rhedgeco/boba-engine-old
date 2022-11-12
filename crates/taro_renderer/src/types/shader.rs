use log::warn;
use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::TaroRenderer;

use super::TaroCompiler;

pub struct CompiledTaroShader {
    pub module: ShaderModule,
}

pub struct TaroShader<'a> {
    descriptor: Option<ShaderModuleDescriptor<'a>>,
    compiled: Option<CompiledTaroShader>,
}

impl<'a> TaroCompiler for TaroShader<'a> {
    type CompiledData = CompiledTaroShader;

    fn get_data(&self) -> &Option<Self::CompiledData> {
        &self.compiled
    }

    fn compile(&mut self, renderer: &TaroRenderer) {
        if self.compiled.is_some() {
            return;
        }

        let Some(render_resources) = renderer.resources() else {
            warn!("Could not compile/upload mesh. TaroRenderer has not been initialized");
            return;
        };

        let descriptor = std::mem::replace(&mut self.descriptor, None)
            .expect("Shader descriptor should be Some at this point");
        let module = render_resources.device.create_shader_module(descriptor);
        self.compiled = Some(CompiledTaroShader { module });
    }
}

impl<'a> TaroShader<'a> {
    pub fn from_str(label: Option<&'a str>, shader_code: &'a str) -> Self {
        let descriptor = ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(shader_code.into()),
        };

        TaroShader::<'a> {
            compiled: None,
            descriptor: Some(descriptor),
        }
    }
}
