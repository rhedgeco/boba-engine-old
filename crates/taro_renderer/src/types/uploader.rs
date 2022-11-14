use crate::RenderResources;

pub trait TaroCompiler {
    type CompiledData;

    fn compile(&mut self, resources: &RenderResources);
    fn get_data(&self) -> &Option<Self::CompiledData>;

    fn get_compiled_data(&mut self, resources: &RenderResources) -> &Self::CompiledData {
        if self.get_data().is_some() {
            return self.get_data().as_ref().unwrap();
        }

        self.compile(resources);
        self.get_data().as_ref().unwrap()
    }
}
