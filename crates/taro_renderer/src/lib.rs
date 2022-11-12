mod render_stage;
mod renderer;
mod resize_controller;

pub mod renderers;
pub mod stages;
pub mod storage;
pub mod types;

pub use render_stage::*;
pub use renderer::*;

pub mod prelude {
    use crate::{
        resize_controller::ResizeController,
        stages::{TaroRenderStage, TaroRendererInitStage},
    };
    use boba_core::*;

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(self, app: &mut boba_core::BobaApp) {
            app.startup_stages().add(TaroRendererInitStage);
            app.stages().add(TaroRenderStage);
            app.events()
                .add_listener(BobaController::build(ResizeController));
        }
    }
}
