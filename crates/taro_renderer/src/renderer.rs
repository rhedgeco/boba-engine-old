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
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    hardware: TaroHardware,
}

impl TaroRenderer {
    pub fn hardware(&self) -> &TaroHardware {
        &self.hardware
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn resize(&mut self, new_size: SurfaceSize) {
        let width = new_size.width;
        let height = new_size.height;
        if width == 0 || height == 0 {
            error!("Error when resizing TaroRenderer to ({width},{height}). All values must be greater than 0.");
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.hardware.device, &self.config);
    }

    pub async fn new<W>(window: W, size: SurfaceSize) -> Self
    where
        W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let hardware = TaroHardware {
            instance,
            adapter,
            device,
            queue,
        };

        Self {
            config,
            surface,
            hardware,
        }
    }
}
