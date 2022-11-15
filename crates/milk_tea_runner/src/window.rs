use std::any::{Any, TypeId};

use winit::window::Window;

pub struct MilkTeaWindows {
    main: Window,
}

impl MilkTeaWindows {
    pub fn new(main_window: Window) -> Self {
        Self { main: main_window }
    }

    pub fn main(&self) -> &Window {
        &self.main
    }

    pub fn main_mut(&mut self) -> &mut Window {
        &mut self.main
    }
}

struct AnySurface {
    typeid: TypeId,
    manager: Box<dyn Any>,
}

pub struct MilkTeaWindow {
    window: Window,
    surface: Option<AnySurface>,
}

impl MilkTeaWindow {
    pub fn new(window: Window) -> Self {
        Self {
            window,
            surface: None,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn has_surface<T>(&mut self) -> bool
    where
        T: 'static + Sized,
    {
        match &self.surface {
            Some(surface) => surface.typeid == TypeId::of::<T>(),
            None => false,
        }
    }

    pub fn set_surface<T>(&mut self, surface: T) -> &T
    where
        T: 'static + Sized,
    {
        self.surface = Some(AnySurface {
            typeid: TypeId::of::<T>(),
            manager: Box::new(surface),
        });

        self.get_surface::<T>().unwrap()
    }

    pub fn get_surface<T>(&mut self) -> Option<&T>
    where
        T: 'static + Sized,
    {
        match &self.surface {
            Some(surface) => surface.manager.downcast_ref::<T>(),
            None => None,
        }
    }
}
