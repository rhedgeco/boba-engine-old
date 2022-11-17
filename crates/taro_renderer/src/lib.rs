mod camera;
mod render_phase;
mod renderer;
mod resize_pearl;
mod window;

pub mod phases;
pub mod renderers;
pub mod stages;
pub mod storage;
pub mod types;

pub use camera::*;
pub use render_phase::*;
pub use renderer::*;
pub use window::*;

pub mod prelude {
    use crate::{resize_pearl::ResizePearl, stages::OnTaroRender};
    use boba_core::*;

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(self, app: &mut boba_core::BobaApp) {
            app.stages.add(OnTaroRender);
            app.events.add_listener(ResizePearl.as_pearl());
        }
    }
}
