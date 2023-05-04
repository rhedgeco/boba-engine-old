use boba_core::Event;

pub struct WindowSpawned {
    pub name: String,
}

impl Event for WindowSpawned {
    type Data<'a> = &'a Self;
}

pub struct WindowClosed {
    pub name: String,
}

impl Event for WindowClosed {
    type Data<'a> = &'a Self;
}

pub struct WindowCloseRequested {
    pub name: String,
}

impl Event for WindowCloseRequested {
    type Data<'a> = &'a Self;
}
