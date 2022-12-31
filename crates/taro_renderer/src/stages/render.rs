use std::marker::PhantomData;

use boba_core::{BobaResources, BobaResult, BobaStage, PearlRegistry};
use log::warn;

use crate::{TaroHardware, TaroRenderPasses, TaroRenderPearls};

pub trait TaroSurfaceManager: 'static {
    fn get_hardware(&self) -> &TaroHardware;
    fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError>;
}

pub struct OnTaroRender<T>
where
    T: TaroSurfaceManager,
{
    _renderer: PhantomData<T>,
}

impl<T> Default for OnTaroRender<T>
where
    T: TaroSurfaceManager,
{
    fn default() -> Self {
        Self {
            _renderer: Default::default(),
        }
    }
}

impl<T> BobaStage for OnTaroRender<T>
where
    T: TaroSurfaceManager,
{
    type Data = ();

    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources) -> BobaResult {
        // run pearls that are listening for this stage
        registry.run_stage::<OnTaroRender<T>>(&(), resources);

        let surface = resources.get::<T>()?;
        let pearls = resources.get::<TaroRenderPearls>()?;
        let mut passes = resources.get_mut::<TaroRenderPasses>()?;
        if passes.len() == 0 {
            warn!("No render passes. Skipping render stage");
            return Ok(());
        }

        let hardware = surface.get_hardware();
        let output = surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            hardware
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        passes.render(&pearls, &view, &mut encoder, hardware);

        hardware.queue().submit(std::iter::once(encoder.finish()));
        Ok(output.present())
    }
}
