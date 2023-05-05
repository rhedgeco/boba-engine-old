use std::{ops::Deref, sync::Arc};

use once_cell::sync::OnceCell;

use crate::TaroHardware;

pub trait TaroCompiler: Sized + 'static {
    type Compiled;
    fn compile(&self, hardware: &TaroHardware) -> Self::Compiled;
}

pub struct Taro<T: TaroCompiler> {
    data: Arc<(T, OnceCell<T::Compiled>)>,
}

impl<T: TaroCompiler> Clone for Taro<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: TaroCompiler> Deref for Taro<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data.0
    }
}

impl<T: TaroCompiler> Taro<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new((data, OnceCell::new())),
        }
    }

    pub fn cached_compile(&self, hardware: &TaroHardware) -> &T::Compiled {
        self.data.1.get_or_init(|| self.data.0.compile(hardware))
    }
}

pub trait TaroExt: TaroCompiler {
    fn into_taro(self) -> Taro<Self>;
}

impl<T: TaroCompiler> TaroExt for T {
    fn into_taro(self) -> Taro<Self> {
        Taro::new(self)
    }
}
