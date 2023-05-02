use boba::prelude::*;

struct WindowListener;

struct WindowSpawner;

impl Pearl for WindowListener {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<WindowSpawn>();
        registrar.listen_for::<WindowDestroy>();
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
        println!("Close Requested for Window '{}'", data.name());
    }
}

impl EventListener<WindowSpawn> for WindowListener {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<WindowSpawn>) {
        println!("Spawned Window '{}'", data.name());
    }
}

impl EventListener<WindowDestroy> for WindowListener {
    fn callback(_: &mut PearlData<Self>, data: BobaEventData<WindowDestroy>) {
        println!("Closed Window '{}'", data.name());
    }
}

impl EventListener<Update> for WindowSpawner {
    fn callback(pearl: &mut PearlData<Self>, mut data: BobaEventData<Update>) {
        let Some(windows) = data.resources.get_mut::<MilkTeaWindows>() else { return };

        let builder1 = WindowBuilder::new()
            .with_title("Spawned Window 1!")
            .with_inner_size(LogicalSize::new(640, 480));

        let builder2 = WindowBuilder::new()
            .with_title("Spawned Window 2!")
            .with_inner_size(LogicalSize::new(640, 480));

        windows.queue_spawn("spawn1", builder1);
        windows.queue_spawn("spawn2", builder2);
        data.pearls.queue_destroy(pearl.handle())
    }
}

fn main() {
    env_logger::init();

    let mut milk_tea = MilkTea::new();
    milk_tea.pearls.insert(WindowListener);
    milk_tea.pearls.insert(WindowSpawner);
    milk_tea.settings.exit_when_close_requested = false;

    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(LogicalSize::new(1280, 800));

    milk_tea.run(window, TaroBuilder::new()).unwrap();
}
