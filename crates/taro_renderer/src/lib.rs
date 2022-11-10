mod renderer;
mod resize_controller;

pub mod renderers;
pub mod stages;
pub mod types;

pub use renderer::*;

pub mod prelude {
    use crate::{
        resize_controller::ResizeController,
        stages::{TaroInitStage, TaroRenderStage},
    };
    use boba_core::*;

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(self, app: &mut boba_core::BobaApp) {
            app.startup_stages().add(TaroInitStage);
            app.stages().add(TaroRenderStage);

            let resize = BobaController::build(ResizeController);
            app.events().add_listener(resize);
        }
    }
}
