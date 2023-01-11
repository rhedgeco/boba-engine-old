use std::marker::PhantomData;

use boba_core::{BobaResources, BobaResult, BobaStage, PearlRegistry, ResourceError};
use log::warn;
use thiserror::Error;

use crate::{TaroCameras, TaroHardware, TaroRenderPearls};

pub trait TaroSurfaceManager: 'static {
    fn get_hardware(&self) -> &TaroHardware;
    fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError>;
    fn get_surface_size(&self) -> (u32, u32);
}

#[derive(Debug, Error)]
#[error("There were no cameras present in the 'TaroCameras' resource")]
pub struct NoCamerasError;

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

        // get rendering manager and cameras
        let surface = resources.get::<T>()?;
        let mut cameras = resources.get_mut::<TaroCameras>()?;
        if cameras.cameras.len() == 0 {
            return Err(NoCamerasError.into());
        }

        // get the graphics hardware
        let hardware = surface.get_hardware();

        // get and create the view for rendering output
        let output = surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        // create the command encoder for rendering
        const COMMAND_ENCODER: wgpu::CommandEncoderDescriptor = wgpu::CommandEncoderDescriptor {
            label: Some("OnTaroRender Command Encoder"),
        };
        let mut encoder = hardware.device().create_command_encoder(&COMMAND_ENCODER);

        // create closure to render all cameras
        let mut render_all_cameras = |pearls: &TaroRenderPearls| {
            let size = surface.get_surface_size();
            for camera in cameras.cameras.iter_mut() {
                camera.resize(size);
                camera.render(&*pearls, &view, &mut encoder, hardware);
            }
        };

        // get pearls to render and use closure to render them
        match resources.get::<TaroRenderPearls>() {
            Ok(pearls) => {
                render_all_cameras(&*pearls);
            }
            Err(ResourceError::NotFound(_)) => {
                warn!("No 'TaroRenderPearls' struct found in resources.");
                render_all_cameras(&TaroRenderPearls::default())
            }
            Err(e) => return Err(e.into()),
        };

        // submit and present the rendered frame
        hardware.queue().submit(std::iter::once(encoder.finish()));
        Ok(output.present())
    }
}
