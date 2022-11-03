use crate::BobaStage;

pub struct BobaEvent<Data: 'static> {
    data: Data,
}

impl<Data: 'static> BobaEvent<Data> {
    pub fn new(data: Data) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl<Data: 'static> BobaStage for BobaEvent<Data> {
    type StageData<'a> = Self;

    fn run(
        &mut self,
        controllers: &mut crate::controller_storage::ControllerStorage,
        resources: &mut crate::BobaResources,
    ) {
        controllers.update::<Self>(self, resources);
    }
}

pub trait BobaEventListener<Data> {
    fn on_trigger(&mut self, data: &Data);
}
