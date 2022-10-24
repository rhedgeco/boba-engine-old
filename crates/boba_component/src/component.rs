use std::{any::TypeId, mem};

#[allow(unused_variables)]
pub trait DataUpdate<T> {
    fn update(&self, item: T) {}
    fn update_mut(&mut self, item: T) {}
}

pub trait RegisteredUpdater {
    unsafe fn try_convert(&self, trait_id: TypeId) -> Option<&(dyn RegisteredUpdater)>;
    unsafe fn try_convert_mut(&mut self, trait_id: TypeId) -> Option<&mut (dyn RegisteredUpdater)>;
}

pub struct Component {
    enabled: bool,
    data: Box<dyn RegisteredUpdater>,
}

impl Component {
    pub fn new<T: 'static + RegisteredUpdater>(data: T) -> Self {
        Self {
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

    pub fn update<T: 'static>(&self, item: T) {
        if let Some(updater) = unsafe {
            self.data
                .as_ref()
                .try_convert(TypeId::of::<dyn DataUpdate<T>>())
                .map(|dst| mem::transmute::<&dyn RegisteredUpdater, &dyn DataUpdate<T>>(dst))
        } {
            updater.update(item);
        }
    }

    pub fn update_mut<T: 'static>(&mut self, item: T) {
        if let Some(updater) = unsafe {
            self.data
                .as_mut()
                .try_convert_mut(TypeId::of::<dyn DataUpdate<T>>())
                .map(|dst| {
                    mem::transmute::<&mut dyn RegisteredUpdater, &mut dyn DataUpdate<T>>(dst)
                })
        } {
            updater.update_mut(item);
        }
    }
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
            /// Transmutes any id into an updater id
            unsafe fn try_convert(&self, trait_id: TypeId) -> Option<&(dyn RegisteredUpdater)> {
                match trait_id {
                    $(
                    id if id == TypeId::of::<dyn DataUpdate<$item>>() => {
                        Some(mem::transmute::<& (dyn DataUpdate<$item>), & dyn RegisteredUpdater>(
                            self as & (dyn DataUpdate<$item>)
                        ))
                    },
                    )*
                    _ => None
                }
            }

            unsafe fn try_convert_mut(&mut self, trait_id: TypeId) -> Option<&mut (dyn RegisteredUpdater)> {
                match trait_id {
                    $(
                    id if id == TypeId::of::<dyn DataUpdate<$item>>() => {
                        Some(mem::transmute::<&mut (dyn DataUpdate<$item>), &mut dyn RegisteredUpdater>(
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
