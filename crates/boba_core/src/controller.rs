use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::BobaResources;

pub struct BobaController<T: 'static + RegisteredStages> {
    pub(crate) controller: Rc<RefCell<T>>,
}

impl<T: 'static + RegisteredStages> Clone for BobaController<T> {
    fn clone(&self) -> Self {
        Self {
            controller: self.controller.clone(),
        }
    }
}

impl<T: 'static + RegisteredStages> BobaController<T> {
    pub fn new(controller: T) -> Self {
        Self {
            controller: Rc::new(RefCell::new(controller)),
        }
    }

    pub fn data(&self) -> Ref<T> {
        self.controller.borrow()
    }

    pub fn data_mut(&mut self) -> RefMut<T> {
        self.controller.borrow_mut()
    }
}

pub trait ControllerStage<StageData: 'static>: RegisteredStages {
    fn update(&mut self, data: &mut StageData, resources: &mut BobaResources);
}

pub trait RegisteredStages {
    unsafe fn transmute_trait(&mut self, trait_id: TypeId) -> Option<&mut dyn RegisteredStages>;
}

#[macro_export]
macro_rules! register_stages {
    ($type:ty: $($item:ty),+ $(,)?) => {

        // weird hack to check if type implements all provided traits
        // uses trait bounds to prevent compilation and show error message
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized $(+ ControllerStage<$item>)+>() {}
            assert_impl_all::<$type>();
        };

        impl RegisteredStages for $type {
            unsafe fn transmute_trait(&mut self, trait_id: std::any::TypeId) -> Option<&mut dyn RegisteredStages> {
                match trait_id {
                    $(
                        id if id == std::any::TypeId::of::<dyn ControllerStage<$item>>() => {
                            Some(std::mem::transmute::<&mut dyn ControllerStage<$item>, &mut dyn RegisteredStages>(
                                self as &mut dyn ControllerStage<$item>
                            ))
                        },
                    )*
                    _ => None,
                }
            }
        }
    };
}
