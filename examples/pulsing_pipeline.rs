use boba::prelude::*;

struct PulsingPipeline {
    speed: f64,
    progress: f64,
}

impl PulsingPipeline {
    pub fn new(speed: f64) -> Self {
        Self {
            speed,
            progress: 0.,
        }
    }
}

impl TaroPipeline for PulsingPipeline {
    fn render(&mut self, _: &Mat4, event: &mut BobaEventData<TaroRender>) {
        let brightness = self.progress * self.progress;
        println!("Brightness: {brightness}");
        let device = event.event.hardware().device();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("White Stage Encoder"),
        });

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("White Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: event.event.output_view(),
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

        event.event.queue_encoder(encoder);

        let Some(time) = event.resources.get::<Time>() else { return };
        self.progress += self.speed * time.delta();
        if self.progress > 1. {
            self.progress = -1. + self.progress.fract();
        }
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();

    let cam_transform = milk_tea
        .pearls
        .insert(Transform::new(TransformData::default()));

    milk_tea.pearls.insert(Taro3DCamera::with_settings(
        cam_transform,
        TaroCameraSettings {
            pipeline: Box::new(PulsingPipeline::new(2.)),
            ..Default::default()
        },
    ));

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
