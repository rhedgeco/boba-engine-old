use std::any::{Any, TypeId};

use winit::window::Window;

pub struct MilkTeaWindows {
    main: MilkTeaWindow,
}

impl MilkTeaWindows {
    pub fn new(main_window: Window) -> Self {
        Self {
            main: MilkTeaWindow::new(main_window),
        }
    }

    pub fn main(&self) -> &MilkTeaWindow {
        &self.main
    }

    pub fn main_mut(&mut self) -> &mut MilkTeaWindow {
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

    pub fn has_surface<T>(&self) -> bool
    where
        T: 'static + Sized,
    {
        match &self.surface {
            Some(surface) => surface.typeid == TypeId::of::<T>(),
            None => false,
        }
    }

    pub fn set_surface<T>(&mut self, surface: T) -> &mut T
    where
        T: 'static + Sized,
    {
        self.surface = Some(AnySurface {
            typeid: TypeId::of::<T>(),
            manager: Box::new(surface),
        });

        self.get_surface::<T>().unwrap()
    }

    pub fn get_surface<T>(&mut self) -> Option<&mut T>
    where
        T: 'static + Sized,
    {
        match &mut self.surface {
            Some(surface) => surface.manager.downcast_mut::<T>(),
            None => None,
        }
    }
}
