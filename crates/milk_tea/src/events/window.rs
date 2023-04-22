pub struct WindowSpawn {
    name: String,
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

impl WindowCloseRequested {
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
