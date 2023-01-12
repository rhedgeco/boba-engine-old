use std::sync::atomic::AtomicU32;

pub struct TaroSurface {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
}

/// The Id for TaroHardware
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HardwareId {
    _id: u32,
}

impl HardwareId {
    fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Self {
            _id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

pub struct TaroHardware {
    id: HardwareId,
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl TaroHardware {
    pub fn id(&self) -> &HardwareId {
        &self.id
    }

    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn build(instance: wgpu::Instance, compatible_surface: &wgpu::Surface) -> Self {
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(compatible_surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                features: wgpu::Features::empty(),
                label: None,
            },
            None,
        ))
        .unwrap();

        Self {
            id: HardwareId::new(),
            instance,
            adapter,
            device,
            queue,
        }
    }

    // pub unsafe fn _build<W>(window: &W, size: (u32, u32)) -> (Self, TaroSurface)
    // where
    //     W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    // {
    //     let instance = wgpu::Instance::new(wgpu::Backends::all());
    //     let surface = unsafe { instance.create_surface(&window) };
    //     let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
    //         power_preference: wgpu::PowerPreference::HighPerformance,
    //         compatible_surface: Some(&surface),
    //         force_fallback_adapter: false,
    //     }))
    //     .unwrap();

    //     let (device, queue) = pollster::block_on(adapter.request_device(
    //         &wgpu::DeviceDescriptor {
    //             limits: if cfg!(target_arch = "wasm32") {
    //                 wgpu::Limits::downlevel_webgl2_defaults()
    //             } else {
    //                 wgpu::Limits::default()
    //             },
    //             features: wgpu::Features::empty(),
    //             label: None,
    //         },
    //         None,
    //     ))
    //     .unwrap();

    //     let config = wgpu::SurfaceConfiguration {
    //         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //         format: surface.get_supported_formats(&adapter)[0],
    //         width: size.0,
    //         height: size.1,
    //         present_mode: wgpu::PresentMode::AutoNoVsync,
    //         alpha_mode: wgpu::CompositeAlphaMode::Auto,
    //     };
    //     surface.configure(&device, &config);

    //     let hardware = Self {
    //         id: HardwareId::new(),
    //         instance,
    //         adapter,
    //         device,
    //         queue,
    //     };

    //     let surface = TaroSurface { surface, config };

    //     (hardware, surface)
    // }
}
