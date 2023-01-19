use std::{
    any::{Any, TypeId},
    cell::{Ref, RefMut},
};

use boba_core::Pearl;
use hashbrown::HashMap;
use indexmap::IndexSet;
use log::error;

use crate::{
    data::{buffers::CameraMatrix, Buffer, Uniform},
    Bind, Taro, TaroHardware,
};

pub trait RenderPipeline: 'static {
    fn render(
        &mut self,
        texture: &RenderTexture,
        pearls: &TaroRenderPearls,
        camera_matrix: &Taro<Bind<Buffer<Uniform<CameraMatrix>>>>,
        hardware: &TaroHardware,
    );
}

pub struct RenderTexture {
    pub size: (u32, u32),
    pub texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
}

#[derive(Default)]
pub struct TaroRenderPearls {
    pearls: HashMap<TypeId, Box<dyn Any>>,
}

impl TaroRenderPearls {
    pub fn add<T: 'static>(&mut self, pearl: Pearl<T>) {
        let typeid = TypeId::of::<T>();
        match self.pearls.get_mut(&typeid) {
            Some(any_set) => {
                any_set
                    .downcast_mut::<IndexSet<Pearl<T>>>()
                    .unwrap()
                    .insert(pearl);
            }
            None => {
                let mut set = IndexSet::<Pearl<T>>::new();
                set.insert(pearl);
                self.pearls.insert(typeid, Box::new(set));
            }
        }
    }

    pub fn remove<T: 'static>(&mut self, pearl: &Pearl<T>) {
        let typeid = TypeId::of::<T>();
        match self.pearls.get_mut(&typeid) {
            Some(any_set) => {
                any_set
                    .downcast_mut::<IndexSet<Pearl<T>>>()
                    .unwrap()
                    .remove(pearl);
            }
            None => (),
        }
    }

    pub fn collect<T: 'static>(&self) -> Vec<Ref<T>> {
        let typeid = TypeId::of::<T>();
        return match self.pearls.get(&typeid) {
            None => Vec::new(),
            Some(any_set) => {
                let set = any_set.downcast_ref::<IndexSet<Pearl<T>>>().unwrap();
                set.iter()
                    .filter_map(|p| match p.borrow() {
                        Ok(data) => Some(data),
                        Err(e) => {
                            let name = std::any::type_name::<T>();
                            error!("Could not collect Pearl<{name}>. Error: {e}");
                            None
                        }
                    })
                    .collect()
            }
        };
    }

    pub fn collect_mut<T: 'static>(&self) -> Vec<RefMut<T>> {
        let typeid = TypeId::of::<T>();
        return match self.pearls.get(&typeid) {
            None => Vec::new(),
            Some(any_set) => {
                let set = any_set.downcast_ref::<IndexSet<Pearl<T>>>().unwrap();
                set.iter()
                    .filter_map(|p| match p.borrow_mut() {
                        Ok(data) => Some(data),
                        Err(e) => {
                            let name = std::any::type_name::<T>();
                            error!("Could not collect Pearl<{name}>. Error: {e}");
                            None
                        }
                    })
                    .collect()
            }
        };
    }
}
