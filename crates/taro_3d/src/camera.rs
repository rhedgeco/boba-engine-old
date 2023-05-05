use boba_3d::{
    glam::{Mat4, Vec3},
    Transform,
};
use boba_core::{
    pearl::map::{Handle, PearlData},
    BobaEventData, EventListener, EventRegistrar, Pearl,
};
use taro_renderer::{
    events::TaroRender,
    milk_tea::{events::Update, Windows},
};

use crate::pipelines::UnlitPipeline;

pub trait TaroPipeline: 'static {
    fn render(&mut self, view_proj_mat: &Mat4, data: &mut BobaEventData<TaroRender>);
}

pub struct TaroCameraSettings {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub target: Option<String>,
    pub pipeline: Box<dyn TaroPipeline>,
}

impl Default for TaroCameraSettings {
    fn default() -> Self {
        Self {
            fovy: 60.0,
            znear: 0.1,
            zfar: 100.0,
            target: Some("main".into()),
            pipeline: Box::new(UnlitPipeline),
        }
    }
}

pub struct Taro3DCamera {
    pub transform: Handle<Transform>,
    pub settings: TaroCameraSettings,
}

impl Taro3DCamera {
    pub fn new(transform: Handle<Transform>) -> Self {
        Self::with_settings(transform, TaroCameraSettings::default())
    }

    pub fn with_settings(transform: Handle<Transform>, settings: TaroCameraSettings) -> Self {
        Self {
            transform,
            settings,
        }
    }

    pub fn has_target(&self, name: &str) -> bool {
        let Some(target) = &self.settings.target else { return false };
        target == name
    }
}

impl Pearl for Taro3DCamera {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
        registrar.listen_for::<TaroRender>();
    }
}

impl EventListener<Update> for Taro3DCamera {
    fn callback(pearl: &mut PearlData<Self>, data: BobaEventData<Update>) {
        let Some(target) = &pearl.settings.target else { return };
        let Some(windows) = data.resources.get_mut::<Windows>() else { return };
        let Some(target_window) = windows.get_window(&target) else { return };
        target_window.request_redraw();
    }
}

impl EventListener<TaroRender> for Taro3DCamera {
    fn callback(pearl: &mut PearlData<Self>, mut data: BobaEventData<TaroRender>) {
        if !pearl.has_target(data.event.window_name()) {
            return;
        }

        let Some(transform) = data.pearls.get(pearl.transform) else { return };
        let (_, rot, pos) = transform.calculate_world_scale_pos_rot();
        let view_mat = Mat4::look_to_rh(pos, rot * Vec3::Z, Vec3::Y);
        let aspect_ratio = data.event.image_width() as f32 / data.event.image_height() as f32;
        let proj_mat = Mat4::perspective_rh(
            pearl.settings.fovy.to_radians(),
            aspect_ratio,
            pearl.settings.znear,
            pearl.settings.zfar,
        );

        let view_proj_mat = proj_mat * view_mat;
        pearl.settings.pipeline.render(&view_proj_mat, &mut data);
    }
}
