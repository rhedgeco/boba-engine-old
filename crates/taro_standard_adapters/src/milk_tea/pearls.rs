use boba_core::{
    BobaResources, BobaResult, Pearl, PearlStage, RegisterPearlStages, StageRegistrar,
};
use milk_tea::events::{MilkTeaEvent, MilkTeaSize};

use super::TaroMilkTea;

pub struct TaroMilkTeaResizeListener;

impl RegisterPearlStages for TaroMilkTeaResizeListener {
    fn register(pearl: Pearl<Self>, stages: &mut impl StageRegistrar) {
        stages.add(pearl.clone());
    }
}

impl PearlStage<MilkTeaEvent<MilkTeaSize>> for TaroMilkTeaResizeListener {
    fn update(
        _pearl: &Pearl<Self>,
        data: &MilkTeaSize,
        resources: &mut BobaResources,
    ) -> BobaResult {
        let mut surface = resources.get_mut::<TaroMilkTea>()?;
        surface.resize(data);
        Ok(())
    }
}
