pub use boba_3d;
pub use boba_core;
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
            KeyCode, KeyboardInput, LateUpdate, MouseMotion, Update, WindowCloseRequested,
            WindowDestroy, WindowSpawn,
        },
        winit::{
            dpi::{LogicalSize, PhysicalSize},
            window::{Fullscreen, WindowBuilder},
        },
        MilkTea, MilkTeaCommands, MilkTeaHeadless, MilkTeaSettings, MilkTeaTime, MilkTeaWindows,
    };
    pub use taro_3d::{TaroCamera, TaroCameraSettings, TaroPipeline, TaroSkybox};
    pub use taro_renderer::{events::TaroRender, wgpu, TaroBuilder};
}
