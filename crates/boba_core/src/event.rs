use crate::{storage::PearlStorage, BobaStage};

pub struct BobaEvent<Data: 'static> {
    pub(crate) data: Data,
}

impl<Data: 'static> BobaEvent<Data> {
    pub fn new(data: Data) -> Self {
        Self { data }
    }
}

impl<Data: 'static> BobaStage for BobaEvent<Data> {
    type StageData = Data;

    fn run(&mut self, _: &mut PearlStorage<Self>, _: &mut crate::BobaResources)
    where
        Self: 'static,
    {
        // do nothing
        // BobaEvent implements BobaStage to take advantage of existing architecture
    }
}
