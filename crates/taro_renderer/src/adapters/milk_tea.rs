use std::ops::Deref;

use boba_core::{
    BobaResources, BobaResult, Pearl, PearlRegistry, PearlStage, RegisterStages, StageCollection,
    StageRegistrar, WrapPearl,
};
use milk_tea::{
    events::{MilkTeaSize, OnMilkTeaResize},
    winit::window::Window,
    MilkTeaAdapter, MilkTeaPlugin,
};

use crate::{SurfaceSize, TaroRenderer};

pub struct TaroMilkTea {
    window: Window,
    renderer: TaroRenderer,
}

impl TaroMilkTea {
    pub fn resize_surface(&mut self, new_size: SurfaceSize) {
        self.renderer.resize_surface(new_size);
    }
}

impl Deref for TaroMilkTea {
    type Target = TaroRenderer;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}

impl MilkTeaAdapter for TaroMilkTea {
    fn build(window: Window) -> Self {
        let size = window.inner_size();
        let renderer = pollster::block_on(TaroRenderer::new(
            &window,
            SurfaceSize {
                width: size.width,
                height: size.height,
            },
        ));

        Self { window, renderer }
    }

    fn raw_window(&self) -> &Window {
        &self.window
    }
}

impl MilkTeaPlugin for TaroMilkTea {
    fn setup(
        registry: &mut PearlRegistry,
        _: &mut StageCollection,
        _: &mut StageCollection,
        _: &mut BobaResources,
    ) {
        registry.add(&ResizeListener.wrap_pearl());
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

        renderer.resize_surface(SurfaceSize {
            width: data.width,
            height: data.height,
        });

        Ok(())
    }
}
