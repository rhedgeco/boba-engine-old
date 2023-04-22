pub use boba_3d;
pub use boba_core;
pub use milk_tea;
pub use taro_3d;
pub use taro_renderer;

pub mod prelude {
    pub use boba_3d::{
        glam::{Quat, Vec3},
        Transform, TransformData,
    };
    pub use boba_core::{
        macros::SimplePearl,
        pearls::map::{BobaPearls, EventData, Handle, PearlData, PearlProvider},
        pearls::{Pearl, PearlId},
        BobaResources, EventListener, EventRegistrar,
    };
    pub use milk_tea::{
        events::{KeyCode, KeyboardInput, LateUpdate, Update},
        winit::{
            dpi::{LogicalSize, PhysicalSize},
            window::WindowBuilder,
        },
        MilkTea, MilkTeaCommands, MilkTeaHeadless, MilkTeaSettings, MilkTeaWindows,
    };
    pub use taro_3d::{TaroCamera, TaroCameraSettings};
    pub use taro_renderer::TaroBuilder;
}
