use crate::{storage::ControllerStorage, BobaStage};

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

    pub fn data_mut(&mut self) -> &mut Data {
        &mut self.data
    }
}

impl<Data: 'static> BobaStage for BobaEvent<Data> {
    type StageData = BobaEvent<Data>;

    fn run(&mut self, _: &mut ControllerStorage<Self>, _: &mut crate::BobaResources)
    where
        Self: 'static,
    {
        // do nothing
        // events implement boba stage to take advantage of existing architecture
    }
}
