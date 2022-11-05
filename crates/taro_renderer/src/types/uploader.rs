use crate::TaroRenderer;

pub trait TaroCompiler {
    type CompiledData;

    fn compile(&mut self, renderer: &TaroRenderer);
    fn get_data(&self) -> &Option<Self::CompiledData>;

    fn get_compiled_data(&mut self, renderer: &TaroRenderer) -> &Self::CompiledData {
        if self.get_data().is_some() {
            return self.get_data().as_ref().unwrap();
        }

        self.compile(renderer);
        self.get_data().as_ref().unwrap()
    }
}
