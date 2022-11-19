use boba_core::Pearl;

use crate::TaroCamera;

#[derive(Default)]
pub struct CameraStorage {
    pub main_camera: Option<Pearl<TaroCamera>>,
}
