use log::error;

pub struct SurfaceSize {
    pub width: u32,
    pub height: u32,
}

pub struct TaroHardware {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct TaroRenderer {
    initial_config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
    hardware: TaroHardware,
}

impl TaroRenderer {
    pub async fn new<W>(window: &W, size: SurfaceSize) -> Self
    where
        W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
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
            )
            .await
            .unwrap();

        let initial_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &initial_config);

        let hardware = TaroHardware {
            instance,
            adapter,
            device,
            queue,
        };

        Self {
            initial_config,
            surface,
            hardware,
        }
    }

    pub fn resize_surface(&self, new_size: SurfaceSize) {
        if new_size.width == 0 && new_size.height == 0 {
            error!(
                "Could not set TaroRenderer surface size to ({},{}).Width and height must be greater than 0.",
                new_size.width, new_size.height
            );
            return;
        }

        let mut new_config = self.initial_config.clone();
        new_config.width = new_size.width;
        new_config.height = new_size.height;
        self.surface.configure(&self.hardware.device, &new_config);
    }

    pub fn hardware(&self) -> &TaroHardware {
        &self.hardware
    }
}
