use std::cell::Ref;

use anymap::AnyMap;
use boba_core::{BobaContainer, BobaController};

use crate::{storage::TaroStorage, CameraStorage};

pub struct RenderResources {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct RenderControllers {
    controllers: AnyMap,
}

impl Default for RenderControllers {
    fn default() -> Self {
        Self {
            controllers: AnyMap::new(),
        }
    }
}

impl RenderControllers {
    pub fn add<T>(&mut self, controller: BobaContainer<T>)
    where
        T: 'static + BobaController,
    {
        match self.controllers.get_mut::<TaroStorage<T>>() {
            Some(storage) => storage.add(controller),
            None => {
                let mut storage = TaroStorage::default();
                storage.add(controller);
                self.controllers.insert(storage);
            }
        }
    }

    pub fn remove<T>(&mut self, controller: BobaContainer<T>)
    where
        T: 'static + BobaController,
    {
        if let Some(storage) = self.controllers.get_mut::<TaroStorage<T>>() {
            storage.remove(controller.uuid());
        }
    }

    pub fn collect<T>(&self) -> Vec<Ref<T>>
    where
        T: 'static + BobaController,
    {
        match self.controllers.get::<TaroStorage<T>>() {
            Some(storage) => storage.collect(),
            None => Vec::new(),
        }
    }
}

pub struct TaroRenderer {
    resources: RenderResources,
    pub cameras: CameraStorage,
    pub controllers: RenderControllers,
}

impl Default for TaroRenderer {
    fn default() -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let resources = RenderResources {
            instance,
            adapter,
            device,
            queue,
        };

        Self {
            resources,
            cameras: Default::default(),
            controllers: Default::default(),
        }
    }
}

impl TaroRenderer {
    pub fn resources(&self) -> &RenderResources {
        &self.resources
    }
}
