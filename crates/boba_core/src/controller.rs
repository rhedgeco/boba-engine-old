use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{BobaResources, BobaStage};

pub struct BobaController<T: 'static + RegisteredStages> {
    pub(crate) controller: Rc<Option<RefCell<T>>>,
}

impl<T: 'static + RegisteredStages> Clone for BobaController<T> {
    fn clone(&self) -> Self {
        Self {
            controller: self.controller.clone(),
        }
    }
}

impl<T: 'static + RegisteredStages> BobaController<T> {
    pub fn build(controller: T) -> Self {
        Self {
            controller: Rc::new(Some(RefCell::new(controller))),
        }
    }

    pub fn data(&self) -> Option<Ref<T>> {
        let Some(test) = self.controller.as_ref() else {
            return None
        };

        Some(test.borrow())
    }

    pub fn data_mut(&mut self) -> Option<RefMut<T>> {
        let Some(test) = self.controller.as_ref() else {
            return None
        };

        Some(test.borrow_mut())
    }
}

pub trait ControllerStage<Stage: 'static + BobaStage>: RegisteredStages {
    fn update<'a>(&'a mut self, data: &mut Stage::StageData<'a>, resources: &mut BobaResources);
}

pub trait RegisteredStages {
    /// # Safety
    ///
    /// This function should only be implemented with the provided macro.
    /// Implementing it yourself could lead to wacky and undefined behaviour. (probably segfaults lol)
    unsafe fn transmute_trait(&mut self, trait_id: TypeId) -> Option<&mut dyn RegisteredStages>;
}

#[macro_export]
macro_rules! register_controller_with_stages {
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
