use boba_3d::{
    glam::{Mat4, Vec3},
    Transform,
};
use boba_core::{
    pearls::{
        map::{EventData, Handle, PearlData},
        Pearl,
    },
    EventListener, EventRegistrar,
};
use taro_renderer::events::TaroRender;

use crate::RenderStages;

pub struct TaroCameraSettings {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub target: Option<String>,
    pub stages: RenderStages,
}

impl Default for TaroCameraSettings {
    fn default() -> Self {
        Self {
            fovy: 60.0,
            znear: 0.1,
            zfar: 100.0,
            target: Some("main".into()),
            stages: Default::default(),
        }
    }
}

pub struct TaroCamera {
    pub transform: Handle<Transform>,
    pub settings: TaroCameraSettings,
}

impl TaroCamera {
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

impl Pearl for TaroCamera {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<TaroRender>();
    }
}

impl EventListener<TaroRender> for TaroCamera {
    fn callback(pearl: &mut PearlData<Self>, mut event: EventData<TaroRender>) {
        if !pearl.has_target(event.window_name()) {
            return;
        }

        let Some(transform) = event.pearls.get(pearl.transform) else { return };
        let (_, rot, pos) = transform.calculate_world_scale_pos_rot();
        let view_mat = Mat4::look_to_rh(pos, rot * Vec3::Z, Vec3::Y);
        let aspect_ratio = event.image_width() as f32 / event.image_height() as f32;
        let proj_mat = Mat4::perspective_rh(
            pearl.settings.fovy.to_radians(),
            aspect_ratio,
            pearl.settings.znear,
            pearl.settings.zfar,
        );

        let view_proj_mat = proj_mat * view_mat;
        pearl.settings.stages.render_all(&view_proj_mat, &mut event);

        event.request_immediate_redraw();
    }
}
