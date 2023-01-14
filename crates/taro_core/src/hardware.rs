use std::sync::atomic::{AtomicU32, Ordering};

/// A unique ID for [`TaroHardware`] structs
///
/// Every `TaroHardware` that is created will get a different ID,
/// even if it is created with the same parameters.
/// This ID is the only way to ensure that a given hardware is different from another.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HardwareId {
    _id: u32,
}

impl HardwareId {
    fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Self {
            _id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

/// Holds all the required structures for creating buffers and doing work on the GPU
pub struct TaroHardware {
    id: HardwareId,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl TaroHardware {
    /// Unique ID for this hardware
    pub fn id(&self) -> &HardwareId {
        &self.id
    }

    /// Graphics device for this hardware
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Graphics queue for this hardware
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

/// A builder to create new [`TaroHardware`] structs
pub struct HardwareBuilder<'a> {
    backends: wgpu::Backends,
    adapter_options: wgpu::RequestAdapterOptions<'a>,
    device_descriptor: wgpu::DeviceDescriptor<'a>,
}

impl<'a> HardwareBuilder<'a> {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            backends: wgpu::Backends::all(),
            adapter_options: wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            },
            device_descriptor: wgpu::DeviceDescriptor {
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                features: wgpu::Features::empty(),
                label: Some("Taro Graphics Device"),
            },
        }
    }

    /// Sets the backends to be used by the builder
    pub fn backends(mut self, backends: wgpu::Backends) -> Self {
        self.backends = backends;
        self
    }

    /// Sets a surface that needs to be compatible with the hardware
    pub fn compatible_surface(mut self, surface: &'a wgpu::Surface) -> Self {
        self.adapter_options.compatible_surface = Some(surface);
        self
    }

    /// Consumes the builder and creates a new `TaroHardware`
    pub fn build(self) -> TaroHardware {
        let id = HardwareId::new();
        let instance = wgpu::Instance::new(self.backends);

        let adapter = pollster::block_on(instance.request_adapter(&self.adapter_options))
            .expect("No valid graphics adapter found.");

        let (device, queue) =
            pollster::block_on(adapter.request_device(&self.device_descriptor, None))
                .expect("Graphics adapter could not be initialized");

        TaroHardware { id, device, queue }
    }
}
