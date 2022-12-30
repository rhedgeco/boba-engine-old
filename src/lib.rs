pub use boba_core as core;
pub use milk_tea;
pub use taro_renderer;

pub mod prelude {
    pub use milk_tea::Bobarista;
    pub use taro_renderer::adapters::TaroMilkTea;

    pub use boba_core::BobaResources;
    pub use boba_core::BobaResult;
    pub use boba_core::BobaStage;
    pub use boba_core::Pearl;
    pub use boba_core::PearlCollector;
    pub use boba_core::PearlRegistry;
    pub use boba_core::PearlStage;
    pub use boba_core::RegisterStages;
    pub use boba_core::ResourceCollector;
    pub use boba_core::StageCollector;
    pub use boba_core::StageRegistrar;
    pub use boba_core::WrapPearl;
}
