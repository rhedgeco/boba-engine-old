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
    fn callback(pearl: &mut PearlData<Self>, mut data: BobaEventData<Update>) {
        let Some(transform) = data.pearls.get(pearl.transform) else { return };
        let location = transform.calculate_world_pos();
        println!("Transform location: {location}");
        data.pearls.queue_destroy(pearl.handle());

        if let Some(commands) = data.resources.get_mut::<Commands>() {
            commands.exit_app();
        }
    }
}

fn main() {
    let mut milk_tea = MilkTeaHeadless::new();

    let transform_base = milk_tea.pearls.insert(Transform::new(TransformData {
        rot: Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()),
        scale: Vec3::X * 2f32,
        ..Default::default()
    }));

    let transform = milk_tea.pearls.insert(Transform::new(TransformData {
        pos: Vec3::X,
        parent: Some(transform_base),
        ..Default::default()
    }));

    milk_tea.pearls.insert(TransformPrinter { transform });

    milk_tea.run();
}
