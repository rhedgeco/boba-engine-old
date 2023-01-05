use indexmap::IndexMap;
use std::any::TypeId;

use crate::{
    passes::BlankRenderPass,
    shading::{buffers::CameraMatrix, TaroBuffer},
    TaroHardware, TaroRenderPearls,
};

pub trait TaroRenderPass: 'static {
    fn render(
        &mut self,
        pearls: &TaroRenderPearls,
        camera_matrix: &TaroBuffer<CameraMatrix>,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        hardware: &TaroHardware,
    );
}

pub struct TaroRenderPasses {
    passes: IndexMap<TypeId, Box<dyn DynamicPassRenderer>>,
}

impl Default for TaroRenderPasses {
    fn default() -> Self {
        let mut new = Self {
            passes: Default::default(),
        };

        new.insert(BlankRenderPass);

        new
    }
}

impl TaroRenderPasses {
    pub fn len(&self) -> usize {
        self.passes.len()
    }

    /// Adds or replaces a pass in the collection.
    ///
    /// If the pass exists, it will be replaced. If it does not it will be appended.
    pub fn insert<Pass>(&mut self, stage: Pass)
    where
        Pass: TaroRenderPass,
    {
        let stageid = TypeId::of::<Pass>();
        self.passes.insert(stageid, Box::new(stage));
    }

    /// Appends a pass to the collection
    ///
    /// If an instance of this pass already exists in this collection, it will be removed first.
    pub fn append<Pass>(&mut self, stage: Pass)
    where
        Pass: TaroRenderPass,
    {
        let stageid = TypeId::of::<Pass>();
        self.passes.shift_remove(&stageid);
        self.passes.insert(stageid, Box::new(stage));
    }

    /// Prepends a pass to the collection
    ///
    /// If an instance of this pass already exists in this collection, it will be removed first.
    pub fn prepend<Pass>(&mut self, stage: Pass)
    where
        Pass: TaroRenderPass,
    {
        let stageid = TypeId::of::<Pass>();
        self.passes.shift_remove(&stageid);

        let (index, _) = self.passes.insert_full(stageid, Box::new(stage));
        if index > 0 {
            self.passes.move_index(index, 0);
        }
    }

    /// Removes a pass from the collection
    pub fn remove<Pass>(&mut self)
    where
        Pass: TaroRenderPass,
    {
        let stageid = TypeId::of::<Pass>();
        self.passes.shift_remove(&stageid);
    }

    /// Renders all the passes in order
    pub fn render(
        &mut self,
        pearls: &TaroRenderPearls,
        camera_matrix: &TaroBuffer<CameraMatrix>,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        hardware: &TaroHardware,
    ) {
        for pass in self.passes.values_mut() {
            pass.dynamic_render(pearls, camera_matrix, view, encoder, hardware);
        }
    }
}

trait DynamicPassRenderer {
    fn type_id(&self) -> TypeId;
    fn dynamic_render(
        &mut self,
        pearls: &TaroRenderPearls,
        camera_matrix: &TaroBuffer<CameraMatrix>,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        hardware: &TaroHardware,
    );
}

impl<Pass> DynamicPassRenderer for Pass
where
    Pass: TaroRenderPass,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<Pass>()
    }

    fn dynamic_render(
        &mut self,
        pearls: &TaroRenderPearls,
        camera_matrix: &TaroBuffer<CameraMatrix>,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        hardware: &TaroHardware,
    ) {
        self.render(pearls, camera_matrix, view, encoder, hardware);
    }
}
