use boba::prelude::*;

struct PulsingBrightness {
    speed: f64,
    progress: f64,
}

impl PulsingBrightness {
    pub fn new(speed: f64) -> Self {
        Self {
            speed,
            progress: 0.,
        }
    }

    pub fn brightness(&self) -> f64 {
        self.progress * self.progress
    }
}

impl Pearl for PulsingBrightness {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for PulsingBrightness {
    fn callback(pearl: &mut PearlData<Self>, event: EventData<Update>) {
        pearl.progress += event.delta_time() * pearl.speed;
        if pearl.progress > 1. {
            pearl.progress = -1. + pearl.progress.fract();
        }
    }
}

struct PulsingRenderStage {
    brightness: Handle<PulsingBrightness>,
}

impl RenderStage for PulsingRenderStage {
    fn render(&mut self, _: &Mat4, event: &mut EventData<TaroRender>) {
        let Some(pearl) = event.pearls.get(self.brightness) else { return };
        let brightness = pearl.brightness();

        let device = event.hardware().device();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("White Stage Encoder"),
        });

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("White Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: event.output_view(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: brightness,
                        g: brightness,
                        b: brightness,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        event.queue_encoder(encoder);
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    let brightness = milk_tea.pearls.insert(PulsingBrightness::new(2.));

    let cam_transform = milk_tea
        .pearls
        .insert(Transform::new(TransformData::default()));

    let mut stages = RenderStages::empty();
    stages.push(PulsingRenderStage { brightness });

    milk_tea.pearls.insert(TaroCamera::with_settings(
        cam_transform,
        TaroCameraSettings {
            stages,
            ..Default::default()
        },
    ));

    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(LogicalSize::new(1280, 800));

    milk_tea.run(window, TaroBuilder::new()).unwrap();
}
