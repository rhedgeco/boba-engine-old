use boba_core::{
    BobaResources, BobaResult, Pearl, PearlRegistry, PearlStage, RegisterStages, StageCollection,
    StageRegistrar, WrapPearl,
};

use milk_tea::{
    stages::{MilkTeaSize, OnMilkTeaResize},
    winit::window::Window,
    MilkTeaAdapter, MilkTeaPlugin,
};

use crate::{stages::OnTaroRender, SurfaceSize, TaroRenderPasses, TaroRenderPearls, TaroRenderer};

pub struct TaroMilkTea {
    window: Window,
}

impl MilkTeaAdapter for TaroMilkTea {
    type Renderer = TaroRenderer;

    fn build(window: &Window) -> Self::Renderer {
        let size = window.inner_size();
        let renderer = pollster::block_on(TaroRenderer::new(
            window,
            SurfaceSize {
                width: size.width,
                height: size.height,
            },
        ));

        renderer
    }

    fn raw_window(&self) -> &Window {
        &self.window
    }
}

impl MilkTeaPlugin for TaroMilkTea {
    fn setup(
        registry: &mut PearlRegistry,
        _: &mut StageCollection,
        main_stages: &mut StageCollection,
        resources: &mut BobaResources,
    ) {
        registry.add(&ResizeListener.wrap_pearl());
        main_stages.append(OnTaroRender);
        resources.add(TaroRenderPearls::default());
        resources.add(TaroRenderPasses::default());
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
        let mut renderer = resources.get_mut::<TaroRenderer>()?;

        renderer.resize(SurfaceSize {
            width: data.width,
            height: data.height,
        });

        Ok(())
    }
}
