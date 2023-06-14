pub use boba_3d;
pub use boba_core;
pub use boba_ecs;
pub use milk_tea;
pub use taro_3d;
pub use taro_renderer;

pub mod prelude {
    pub use boba_3d::{
        glam::{Mat4, Quat, Vec3},
        Transform, TransformData,
    };
    pub use boba_core::{
        pearl::{
            map::{Handle, PearlData},
            PearlProvider,
        },
        BobaEventData, BobaPearls, BobaResources, EventListener, EventRegistrar, Pearl,
    };
    pub use milk_tea::{
        events::{
            KeyCode, KeyboardInput, LateUpdate, MouseMotion, Time, Update, WindowCloseRequested,
            WindowClosed, WindowSpawned,
        },
        winit::{
            dpi::{LogicalSize, PhysicalSize},
            window::{Fullscreen, WindowBuilder},
        },
        Commands, MilkTea, MilkTeaHeadless, MilkTeaSettings, WindowSettings, Windows,
    };
    pub use taro_3d::{Taro3DCamera, TaroCameraSettings, TaroPipeline, TaroSkybox};
    pub use taro_renderer::{events::TaroRender, wgpu, TaroBuilder};
}
