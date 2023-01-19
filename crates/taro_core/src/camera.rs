use boba_3d::{
    glam::{Mat4, Quat, Vec3},
    pearls::BobaTransform,
};
use boba_core::Pearl;

use crate::{
    data::{buffers::CameraMatrix, UniformBuffer},
    Bind, Taro,
};

/// Settings for [`TaroCamera`]
#[derive(Debug, Clone)]
pub struct TaroCameraSettings {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for TaroCameraSettings {
    fn default() -> Self {
        Self {
            fovy: 60.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

/// Core struct to render images to a [`RenderTexture`]
pub struct TaroCamera {
    aspect_ratio: f32,
    camera_matrix: Taro<Bind<UniformBuffer<CameraMatrix>>>,

    pub transform: Pearl<BobaTransform>,
    pub settings: TaroCameraSettings,
}

impl TaroCamera {
    /// Creates a new camera with `transform`
    pub fn new_simple(transform: BobaTransform) -> Self {
        Self::new_with_settings(Pearl::wrap(transform), TaroCameraSettings::default())
    }

    /// Creates a new camera with `transform` and `settings`
    pub fn new_with_settings(
        transform: Pearl<BobaTransform>,
        settings: TaroCameraSettings,
    ) -> Self {
        let camera_matrix = UniformBuffer::new(wgpu::BufferUsages::UNIFORM);
        let camera_matrix = Bind::new(camera_matrix);

        Self {
            aspect_ratio: 1.,
            camera_matrix,
            transform,
            settings,
        }
    }

    /// Renders the camera to the given `texture` using the provided `pearls` and `hardware`
    // pub fn render(
    //     &mut self,
    //     texture: &RenderTexture,
    //     pearls: &TaroRenderPearls,
    //     hardware: &TaroHardware,
    // ) {
    //     self.aspect_ratio = texture.size.0 as f32 / texture.size.1 as f32;
    //     match self.transform.borrow() {
    //         Ok(t) => {
    //             self.camera_matrix.write_to_hardware(
    //                 &Self::calculate_matrix(
    //                     t.world_position(),
    //                     t.world_rotation(),
    //                     self.aspect_ratio,
    //                     &self.settings,
    //                 )
    //                 .into()
    //                 .into(),
    //                 hardware,
    //             );
    //         }
    //         Err(e) => {
    //             error!("Error when calculating camera matrix. Error: {e}");
    //         }
    //     };

    //     self.pipeline
    //         .render(texture, pearls, &self.camera_matrix, hardware);
    // }

    /// Calculates a new [`Mat4`] that represents a view projection matrix with the given transforms
    pub fn calculate_matrix(
        position: Vec3,
        rotation: Quat,
        aspect: f32,
        settings: &TaroCameraSettings,
    ) -> Mat4 {
        let target = position + rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(
            settings.fovy.to_radians(),
            aspect,
            settings.znear,
            settings.zfar,
        );

        proj * view
    }
}
