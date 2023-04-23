use boba::prelude::*;

struct PulsingRenderStage {
    progress: f64,
}

impl RenderStage for PulsingRenderStage {
    fn render(&mut self, _: &Mat4, event: &mut EventData<TaroRender>) {
        let device = event.hardware().device();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("White Stage Encoder"),
        });

        let brightness = self.progress * self.progress;

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
        self.progress += 0.001;
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

    let mut stages = RenderStages::empty();
    stages.push(PulsingRenderStage { progress: 0.5 });

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
