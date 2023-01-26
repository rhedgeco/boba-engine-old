use boba_3d::pearls::BobaTransform;
use boba_core::Pearl;
use log::error;

use crate::{
    data::{buffers::CameraMatrix, Buffer, Uniform},
    rendering::{RenderPipeline, RenderTexture, TaroRenderPearls},
    Bind, Taro, TaroHardware,
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
    camera_matrix: Taro<Bind<Buffer<Uniform<CameraMatrix>>>>,
    pipeline: Box<dyn RenderPipeline>,

    pub transform: Pearl<BobaTransform>,
    pub settings: TaroCameraSettings,
}

impl TaroCamera {
    /// Creates a new camera with `transform`
    pub fn new_simple(transform: BobaTransform, pipeline: impl RenderPipeline) -> Self {
        Self::new_with_settings(
            Pearl::wrap(transform),
            TaroCameraSettings::default(),
            pipeline,
        )
    }

    /// Creates a new camera with `transform` and `settings`
    pub fn new_with_settings(
        transform: Pearl<BobaTransform>,
        settings: TaroCameraSettings,
        pipeline: impl RenderPipeline,
    ) -> Self {
        Self {
            aspect_ratio: 1.,
            camera_matrix: Bind::new(Buffer::new(wgpu::BufferUsages::UNIFORM)),
            pipeline: Box::new(pipeline),
            transform,
            settings,
        }
    }

    /// Replaces the cameras current [`RenderPipeline`] with a new `pipeline`
    pub fn set_pipeline(&mut self, pipeline: impl RenderPipeline) {
        self.pipeline = Box::new(pipeline)
    }

    /// Renders the camera to the given `texture` using the provided `pearls` and `hardware`
    pub fn render(
        &mut self,
        texture: &RenderTexture,
        pearls: &TaroRenderPearls,
        hardware: &TaroHardware,
    ) {
        let size = texture.size();
        self.aspect_ratio = size.0 as f32 / size.1 as f32;
        match self.transform.borrow() {
            Ok(t) => {
                self.camera_matrix.bind_data().write_to_hardware(
                    CameraMatrix::new(
                        t.world_position(),
                        t.world_rotation(),
                        self.aspect_ratio,
                        &self.settings,
                    )
                    .into(),
                    hardware,
                );
            }
            Err(e) => {
                error!("Error when calculating camera matrix. Error: {e}");
            }
        };

        self.pipeline
            .render(texture, pearls, &self.camera_matrix, hardware);
    }
}
