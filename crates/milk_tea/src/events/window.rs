use winit::window::WindowId;

pub struct WindowSpawn {
    id: WindowId,
    name: String,
}

impl WindowSpawn {
    pub(crate) fn new(id: WindowId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct WindowDrop {
    id: WindowId,
    name: String,
}

impl WindowDrop {
    pub(crate) fn new(id: WindowId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
