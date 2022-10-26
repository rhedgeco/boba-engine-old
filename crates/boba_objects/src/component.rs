use std::{
    any::TypeId,
    mem::{self, transmute, transmute_copy},
};

use uuid::Uuid;

pub struct BobaComponent {
    pub(crate) uuid: Uuid,
    enabled: bool,
    data: Box<dyn RegisteredUpdater>,
}

impl BobaComponent {
    pub(crate) fn new<T: 'static + RegisteredUpdater>(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            enabled: true,
            data: Box::new(data),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled
    }

    pub fn update<T: 'static>(&mut self, item: &T) {
        if let Some(updater) = unsafe {
            self.data
                .as_mut()
                .try_convert(TypeId::of::<dyn DataUpdate<T>>())
                .map(|dst| {
                    mem::transmute::<&mut dyn RegisteredUpdater, &mut dyn DataUpdate<T>>(dst)
                })
        } {
            updater.update(item);
        }
    }

    pub fn get_data<T: 'static>(&self) -> Option<&T> {
        if self.data.data_is(TypeId::of::<T>()) {
            Some(unsafe {
                let trait_object: TraitObject = transmute_copy(&self.data.as_ref());
                transmute(trait_object.data)
            })
        } else {
            None
        }
    }

    pub fn get_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if self.data.data_is(TypeId::of::<T>()) {
            Some(unsafe {
                let trait_object: TraitObject = transmute_copy(&self.data.as_mut());
                transmute(trait_object.data)
            })
        } else {
            None
        }
    }
}

/// Used for extracting the data part of a trait.
/// This can be useful when trying to transmute a trait back to its struct.
struct TraitObject {
    pub data: *mut (),
    pub _vtable: *mut (),
}

#[allow(unused_variables)]
pub trait DataUpdate<T>: RegisteredUpdater {
    fn update(&mut self, item: &T) {}
}

pub trait RegisteredUpdater {
    fn data_is(&self, data_id: TypeId) -> bool;
    unsafe fn try_convert(&mut self, trait_id: TypeId) -> Option<&mut (dyn RegisteredUpdater)>;
}

#[macro_export]
macro_rules! register_updaters {
    ($type:ty: $($item:ty),+ $(,)?) => {

        // weird hack to check if type implements all provided traits
        // uses trait bounds to prevent compilation and show error message
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized $(+ DataUpdate<$item>)+>() {}
            assert_impl_all::<$type>();
        };

        // implement RegisteredUpdater for generic storage
        impl RegisteredUpdater for $type {
            fn data_is(&self, data_id: std::any::TypeId) -> bool {
                std::any::TypeId::of::<$type>() == data_id
            }

            unsafe fn try_convert(&mut self, trait_id: std::any::TypeId) -> Option<&mut (dyn RegisteredUpdater)> {
                match trait_id {
                    $(
                    id if id == std::any::TypeId::of::<dyn DataUpdate<$item>>() => {
                        Some(std::mem::transmute::<&mut (dyn DataUpdate<$item>), &mut dyn RegisteredUpdater>(
                            self as &mut (dyn DataUpdate<$item>)
                        ))
                    },
                    )*
                    _ => None
                }
            }
        }
    };
}
