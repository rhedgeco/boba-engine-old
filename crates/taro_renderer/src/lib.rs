mod renderer;

pub mod stages;

pub use renderer::*;

pub mod prelude {
    use boba_core::BobaPlugin;

    use crate::stages::{TaroRenderStage, TaroStartup};

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(&self, app: &mut boba_core::BobaApp) {
            app.startup_stages().add(TaroStartup);
            app.stages().add(TaroRenderStage);
        }
    }
}
