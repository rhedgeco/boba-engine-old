use std::{ops::Deref, sync::Arc};

use once_map::OnceMap;

use crate::{HardwareId, TaroHardware};

/// Required trait to be built into a [`Taro`] object
pub trait Compiler: 'static {
    type Compiled;
    fn new_taro_compile(&self, hardware: &TaroHardware) -> Self::Compiled;
}

/// Manages and compiles data associated with a given [`TaroHardware`]
pub struct Taro<T: Compiler> {
    data: Arc<T>,
    cache: OnceMap<HardwareId, T::Compiled>,
}

impl<T: Compiler> Clone for Taro<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl<T: Compiler> Deref for Taro<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Compiler> Taro<T> {
    /// Creates a new taro object containing `data`
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(data),
            cache: Default::default(),
        }
    }

    /// Gets or compiles a new instance of `T` associated with a given `hardware`
    pub fn get_or_compile(&self, hardware: &TaroHardware) -> &T::Compiled {
        self.cache
            .get_or_init(hardware.id().clone(), || {
                self.data.new_taro_compile(hardware)
            })
            .into_data()
    }
}
