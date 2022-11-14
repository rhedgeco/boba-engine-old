use std::borrow::Cow;

use log::warn;
use naga::{front::wgsl::ParseError, Module};
use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::TaroRenderer;

use super::TaroCompiler;

pub struct CompiledTaroShader {
    pub module: ShaderModule,
}

pub struct TaroShader {
    label: Box<str>,
    module: Module,
    compiled: Option<CompiledTaroShader>,
}

impl<'a> TaroCompiler for TaroShader {
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

        let descriptor = ShaderModuleDescriptor {
            label: Some(&self.label),
            source: wgpu::ShaderSource::Naga(Cow::Owned(self.module.clone())),
        };

        let module = render_resources.device.create_shader_module(descriptor);
        self.compiled = Some(CompiledTaroShader { module });
    }
}

impl TaroShader {
    pub fn from_wgsl(label: &str, source: &str) -> Result<Self, ParseError> {
        Ok(Self {
            label: Box::<str>::from(label),
            module: naga::front::wgsl::parse_str(source)?,
            compiled: None,
        })
    }
}
