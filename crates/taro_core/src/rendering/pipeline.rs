use crate::{
    data::{buffers::CameraMatrix, Buffer, Uniform},
    Bind, Taro, TaroHardware,
};

use super::{RenderTexture, TaroRenderPearls};

pub trait RenderPipeline: 'static {
    fn render(
        &mut self,
        texture: &RenderTexture,
        pearls: &TaroRenderPearls,
        camera_matrix: &Taro<Bind<Buffer<Uniform<CameraMatrix>>>>,
        hardware: &TaroHardware,
    );
}
