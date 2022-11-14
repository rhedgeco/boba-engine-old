mod camera;
mod render_phase;
mod renderer;
mod resize_controller;

pub mod phases;
pub mod renderers;
pub mod stages;
pub mod storage;
pub mod types;

pub use camera::*;
pub use render_phase::*;
pub use renderer::*;

pub mod prelude {
    use crate::{
        resize_controller::ResizeController,
        stages::{OnTaroRender, TaroRendererInitStage},
    };
    use boba_core::*;

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(self, app: &mut boba_core::BobaApp) {
            app.startup_stages().add(TaroRendererInitStage);
            app.stages().add(OnTaroRender);
            app.events()
                .add_listener(BobaContainer::build(ResizeController));
        }
    }
}
