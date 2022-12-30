use std::ops::{Deref, DerefMut};

use boba_core::{
    BobaResources, BobaResult, Pearl, PearlRegistry, PearlStage, RegisterStages, StageCollection,
    StageRegistrar, WrapPearl,
};

use milk_tea::{
    stages::{MilkTeaSize, OnMilkTeaResize},
    winit::window::Window,
    MilkTeaAdapter, MilkTeaPlugin,
};

use crate::{stages::OnTaroRender, SurfaceSize, TaroRenderer};

pub struct TaroMilkTea {
    renderer: TaroRenderer<Window>,
}

impl Deref for TaroMilkTea {
    type Target = TaroRenderer<Window>;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}

impl DerefMut for TaroMilkTea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.renderer
    }
}

impl MilkTeaAdapter for TaroMilkTea {
    fn build(window: Window) -> Self {
        let size = window.inner_size();
        let renderer = pollster::block_on(TaroRenderer::new(
            window,
            SurfaceSize {
                width: size.width,
                height: size.height,
            },
        ));

        Self { renderer }
    }

    fn raw_window(&self) -> &Window {
        &self.renderer.window()
    }
}

impl MilkTeaPlugin for TaroMilkTea {
    fn setup(
        registry: &mut PearlRegistry,
        _: &mut StageCollection,
        main_stages: &mut StageCollection,
        _: &mut BobaResources,
    ) {
        registry.add(&ResizeListener.wrap_pearl());
        main_stages.append(OnTaroRender);
    }
}

struct ResizeListener;

impl RegisterStages for ResizeListener {
    fn register(pearl: &Pearl<Self>, stages: &mut impl StageRegistrar) {
        stages.add(pearl.clone());
    }
}

impl PearlStage<OnMilkTeaResize> for ResizeListener {
    fn update(&mut self, data: &MilkTeaSize, resources: &mut BobaResources) -> BobaResult {
        let mut renderer = resources.get_mut::<TaroMilkTea>()?;

        renderer.resize(SurfaceSize {
            width: data.width,
            height: data.height,
        });

        Ok(())
    }
}
