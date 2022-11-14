use std::borrow::Cow;

use naga::{front::wgsl::ParseError, Module};
use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::RenderResources;

pub struct CompiledTaroShader {
    pub module: ShaderModule,
}

pub struct TaroShader {
    label: Box<str>,
    module: Module,
    compiled: Option<CompiledTaroShader>,
}

impl TaroShader {
    pub fn from_wgsl(label: &str, source: &str) -> Result<Self, ParseError> {
        Ok(Self {
            label: Box::<str>::from(label),
            module: naga::front::wgsl::parse_str(source)?,
            compiled: None,
        })
    }

    pub fn get_compiled(&self) -> &Option<CompiledTaroShader> {
        &self.compiled
    }

    pub fn compile(&mut self, resources: &RenderResources) -> &CompiledTaroShader {
        if self.compiled.is_some() {
            return self.compiled.as_ref().unwrap();
        }

        let descriptor = ShaderModuleDescriptor {
            label: Some(&self.label),
            source: wgpu::ShaderSource::Naga(Cow::Owned(self.module.clone())),
        };

        let module = resources.device.create_shader_module(descriptor);
        self.compiled = Some(CompiledTaroShader { module });
        return self.compiled.as_ref().unwrap();
    }
}
