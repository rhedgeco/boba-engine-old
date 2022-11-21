use std::sync::atomic::AtomicU64;

use crate::RenderResources;

pub trait TaroShaderCore {
    fn prepare(&mut self, resources: &RenderResources);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ShaderId {
    _id: u64,
}

impl ShaderId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            _id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

pub trait WrapShader
where
    Self: TaroShaderCore + Sized,
{
    fn wrap(self) -> TaroShader<Self>;
}

impl<T: TaroShaderCore> WrapShader for T {
    fn wrap(self) -> TaroShader<Self> {
        TaroShader {
            id: ShaderId::new(),
            shader: self,
        }
    }
}

pub struct TaroShader<T>
where
    T: TaroShaderCore,
{
    id: ShaderId,
    pub shader: T,
}

impl<T> TaroShader<T>
where
    T: TaroShaderCore,
{
    pub fn id(&self) -> ShaderId {
        self.id
    }
}
