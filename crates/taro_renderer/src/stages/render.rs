use boba_core::{storage::ControllerStorage, BobaStage};
use log::{error, warn};
use milk_tea_runner::MilkTeaWindows;

use crate::{TaroRenderer, TaroWindowSurface};

pub struct OnTaroRender;

impl BobaStage for OnTaroRender {
    type StageData = ();

    fn run(
        &mut self,
        controllers: &mut ControllerStorage<Self>,
        resources: &mut boba_core::BobaResources,
    ) {
        let renderer = match resources.borrow::<TaroRenderer>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderStage. TaroRenderer Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        let mut windows = match resources.borrow_mut::<MilkTeaWindows>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderStage. MilkTeaWindows Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        let main_window = windows.main_mut();

        let taro_surface = match main_window.get_surface::<TaroWindowSurface>() {
            Some(s) => s,
            None => {
                main_window.set_surface(TaroWindowSurface::new(main_window.window(), &*renderer))
            }
        };

        let output = match taro_surface.surface.get_current_texture() {
            Ok(surface) => surface,
            Err(surface_error) => {
                error!(
                    "Skipping TaroRenderStage. Could not get current surface texture. SurfaceError: {:?}",
                    surface_error
                );
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            renderer
                .resources()
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        drop(windows);
        drop(renderer); // drop renderer so that resources may be passed as mutable to controllers
        controllers.update(&(), resources);

        let renderer = match resources.borrow::<TaroRenderer>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderStage. TaroRenderer Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        if let Some(camera_container) = &renderer.cameras.main_camera {
            if let Ok(mut camera) = camera_container.data().try_borrow_mut() {
                camera.rebuild_matrix(renderer.resources());
                camera.execute_render_phases(&view, &mut encoder, &renderer.controllers);
            } else {
                error!("Could not render main camera. It is currently borrowed as mutable.");
            }
        }

        renderer
            .resources()
            .queue
            .submit(std::iter::once(encoder.finish()));

        output.present();
    }
}
