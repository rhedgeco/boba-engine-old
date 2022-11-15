use std::cell::Ref;

use anymap::AnyMap;
use boba_core::{BobaContainer, BobaController};
use wgpu::{CommandEncoder, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{storage::TaroStorage, RenderPhaseStorage, TaroCamera};

pub struct RenderResources {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
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

#[derive(Default)]
pub struct TaroRenderer {
    resources: Option<RenderResources>,
    controllers: RenderControllers,
    phases: RenderPhaseStorage,
}

impl TaroRenderer {
    pub fn initialize(&mut self, window: &Window) {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *surface
                .get_supported_formats(&adapter)
                .get(0)
                .expect("There were no supported adapter formats"),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        let resources = RenderResources {
            surface,
            device,
            queue,
            config,
            size,
        };

        self.resources = Some(resources);
    }

    pub fn resources(&self) -> &Option<RenderResources> {
        &self.resources
    }

    pub fn render_controllers(&mut self) -> &mut RenderControllers {
        &mut self.controllers
    }

    pub fn render_phases(&mut self) -> &mut RenderPhaseStorage {
        &mut self.phases
    }

    pub fn execute_render_phases(
        &mut self,
        view: &TextureView,
        camera: &TaroCamera,
        encoder: &mut CommandEncoder,
    ) {
        self.phases
            .execute_phases(view, camera, encoder, &mut self.controllers);
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        let Some(resources) = &mut self.resources else {
            return;
        };

        if new_size.width > 0 && new_size.height > 0 {
            resources.size = new_size;
            resources.config.width = new_size.width;
            resources.config.height = new_size.height;
            resources
                .surface
                .configure(&resources.device, &resources.config);
        }
    }
}
