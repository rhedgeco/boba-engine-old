use std::cell::Ref;

use anymap::AnyMap;
use boba_core::Pearl;

use crate::{storage::TaroStorage, CameraStorage};

pub struct RenderResources {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct RenderPearls {
    pearls: AnyMap,
}

impl Default for RenderPearls {
    fn default() -> Self {
        Self {
            pearls: AnyMap::new(),
        }
    }
}

impl RenderPearls {
    pub fn add<T>(&mut self, pearl: Pearl<T>)
    where
        T: 'static,
    {
        match self.pearls.get_mut::<TaroStorage<T>>() {
            Some(storage) => storage.add(pearl),
            None => {
                let mut storage = TaroStorage::default();
                storage.add(pearl);
                self.pearls.insert(storage);
            }
        }
    }

    pub fn remove<T>(&mut self, pearl: Pearl<T>)
    where
        T: 'static,
    {
        if let Some(storage) = self.pearls.get_mut::<TaroStorage<T>>() {
            storage.remove(pearl.uuid());
        }
    }

    pub fn collect<T>(&self) -> Vec<Ref<T>>
    where
        T: 'static,
    {
        match self.pearls.get::<TaroStorage<T>>() {
            Some(storage) => storage.collect(),
            None => Vec::new(),
        }
    }
}

pub struct TaroRenderer {
    resources: RenderResources,
    pub cameras: CameraStorage,
    pub pearls: RenderPearls,
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
            pearls: Default::default(),
        }
    }
}

impl TaroRenderer {
    pub fn resources(&self) -> &RenderResources {
        &self.resources
    }
}
