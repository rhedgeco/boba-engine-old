mod camera;
mod render_phase;
mod renderer;
mod resize_pearl;
mod window_surface;

pub mod renderers;
pub mod shading;
pub mod stages;
pub mod storage;
pub mod types;

pub use camera::*;
pub use render_phase::*;
pub use renderer::*;
pub use window_surface::*;

pub use wgpu;

pub mod prelude {
    use crate::{resize_pearl::ResizePearl, stages::OnTaroRender};
    use boba_core::*;

    pub struct TaroRenderPlugin;

    impl BobaPlugin for TaroRenderPlugin {
        fn setup(self, app: &mut boba_core::BobaApp) {
            app.stages.insert(OnTaroRender);
            app.events.add_listener(ResizePearl.into_pearl());
        }
    }
}
