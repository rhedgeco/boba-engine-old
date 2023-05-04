use boba::prelude::*;

struct WindowListener;

struct WindowSpawner;

impl Pearl for WindowListener {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<WindowSpawned>();
        registrar.listen_for::<WindowClosed>();
        registrar.listen_for::<WindowCloseRequested>();
    }
}

impl Pearl for WindowSpawner {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<Update>();
    }
}

impl EventListener<WindowCloseRequested> for WindowListener {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<WindowCloseRequested>) {
        println!("Close Requested for Window '{}'", data.event.name);
    }
}

impl EventListener<WindowSpawned> for WindowListener {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<WindowSpawned>) {
        println!("Spawned Window '{}'", data.event.name);
    }
}

impl EventListener<WindowClosed> for WindowListener {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<WindowClosed>) {
        println!("Closed Window '{}'", data.event.name);
    }
}

impl EventListener<Update> for WindowSpawner {
    fn callback(pearl: &mut PearlData<Self>, mut data: BobaEventData<Update>) {
        let Some(windows) = data.resources.get_mut::<Windows>() else { return };

        let settings1 = WindowSettings {
            title: "Spawned Window 1!".into(),
            size: (640, 480),
        };

        let settings2 = WindowSettings {
            title: "Spawned Window 2!".into(),
            size: (640, 480),
        };

        windows.queue_spawn("spawn1", settings1);
        windows.queue_spawn("spawn2", settings2);
        data.pearls.queue_destroy(pearl.handle())
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(WindowListener);
    milk_tea.pearls.insert(WindowSpawner);
    milk_tea.settings.exit_on_close = false;

    milk_tea
        .run(WindowSettings::default(), TaroBuilder::new())
        .unwrap();
}
