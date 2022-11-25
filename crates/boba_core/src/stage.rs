use crate::{storage::PearlStorage, BobaResources};

pub trait BobaStage: 'static {
    type StageData;
    fn run(&mut self, pearls: &mut PearlStorage<Self>, resources: &mut BobaResources);
}

pub(crate) struct StageRunner<Stage>
where
    Stage: BobaStage,
{
    stage: Stage,
    pub pearls: PearlStorage<Stage>,
}

impl<Stage> StageRunner<Stage>
where
    Stage: BobaStage,
{
    pub fn build(stage: Stage) -> Self {
        Self {
            stage,
            pearls: Default::default(),
        }
    }

    pub fn run(&mut self, resources: &mut BobaResources) {
        self.stage.run(&mut self.pearls, resources);
    }
}

pub struct MainBobaUpdate;

impl BobaStage for MainBobaUpdate {
    type StageData = f32;

    fn run(&mut self, pearls: &mut PearlStorage<Self>, resources: &mut BobaResources) {
        pearls.update(&resources.time().delta(), resources);
    }
}
