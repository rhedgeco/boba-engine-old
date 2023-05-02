use boba_core::Event;

pub struct WindowSpawn {
    name: String,
}

impl Event for WindowSpawn {
    type Data<'a> = Self;
}

impl WindowSpawn {
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct WindowDestroy {
    name: String,
}

impl Event for WindowDestroy {
    type Data<'a> = Self;
}

impl WindowDestroy {
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct WindowCloseRequested {
    name: String,
}

impl Event for WindowCloseRequested {
    type Data<'a> = Self;
}

impl WindowCloseRequested {
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
