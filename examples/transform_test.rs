use boba::prelude::*;

struct TransformPrinter {
    transform: Handle<Transform>,
}

impl Pearl for TransformPrinter {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<Update> for TransformPrinter {
    fn callback(pearl: &mut PearlData<Self>, mut event: EventData<Update>) {
        let Some(transform) = event.pearls.get(pearl.transform) else { return };
        let location = transform.calculate_world_pos();
        println!("Transform location: {location}");
        event.pearls.queue_destroy(pearl.handle());

        if let Some(commands) = event.resources.get_mut::<MilkTeaCommands>() {
            commands.exit_app();
        }
    }
}

fn main() {
    let mut pearls = BobaPearls::new();

    let transform_base = pearls.insert(Transform::new(TransformData {
        rot: Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()),
        scale: Vec3::X * 2f32,
        ..Default::default()
    }));

    let transform = pearls.insert(Transform::new(TransformData {
        pos: Vec3::X,
        parent: Some(transform_base),
        ..Default::default()
    }));

    pearls.insert(TransformPrinter { transform });

    MilkTeaHeadless::run(pearls, BobaResources::new());
}
