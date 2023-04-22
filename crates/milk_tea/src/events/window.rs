pub struct WindowSpawn {
    name: String,
}

impl WindowSpawn {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct WindowDestroy {
    name: String,
}

impl WindowDestroy {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct CloseRequested {
    name: String,
}

impl CloseRequested {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
