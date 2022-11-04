mod renderer;
mod resize_controller;
mod texture;

pub mod renderers;
pub mod stages;

pub use renderer::*;
pub use texture::*;

pub mod prelude {
    use crate::{
        resize_controller::ResizeController,
        stages::{TaroRenderStage, TaroStartup},
    };
    use boba_core::*;

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(&self, app: &mut boba_core::BobaApp) {
            app.startup_stages().add(TaroStartup);
            app.stages().add(TaroRenderStage);
            app.controllers()
                .add(BobaController::build(ResizeController));
        }
    }
}
