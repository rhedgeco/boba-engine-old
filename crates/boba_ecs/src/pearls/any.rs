use imposters::{collections::vec::ImposterVec, Imposter};

use super::{Pearl, PearlId};

pub struct AnyPearl {
    imposter: Imposter,
}

impl AnyPearl {
    #[inline]
    pub fn new<T: Pearl>(pearl: T) -> Self {
        Self {
            imposter: Imposter::new(pearl),
        }
    }

    #[inline]
    pub fn id(&self) -> PearlId {
        PearlId(self.imposter.type_id())
    }

    #[inline]
    pub fn downcast<T: Pearl>(self) -> Option<T> {
        self.imposter.downcast::<T>()
    }

    #[inline]
    pub fn downcast_ref<T: Pearl>(&self) -> Option<&T> {
        self.imposter.downcast_ref::<T>()
    }

    #[inline]
    pub fn downcast_mut<T: Pearl>(&mut self) -> Option<&mut T> {
        self.imposter.downcast_mut::<T>()
    }
}

pub struct AnyPearlVec {
    imposter: ImposterVec,
}

impl AnyPearlVec {
    #[inline]
    pub fn new<T: Pearl>() -> Self {
        Self {
            imposter: ImposterVec::new::<T>(),
        }
    }

    #[inline]
    pub fn from_any(pearl: AnyPearl) -> Self {
        Self {
            imposter: ImposterVec::from_imposter(pearl.imposter),
        }
    }

    #[inline]
    pub fn push_pearl<T: Pearl>(&mut self, pearl: T) -> Result<(), T> {
        self.imposter.push_item(pearl)
    }

    #[inline]
    pub fn push_any(&mut self, pearl: AnyPearl) -> Result<(), AnyPearl> {
        match self.imposter.push_imposter(pearl.imposter) {
            Err(imposter) => Err(AnyPearl { imposter }),
            _ => Ok(()),
        }
    }

    #[inline]
    pub fn swap_drop(&mut self, index: usize) -> bool {
        self.imposter.swap_drop(index)
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> Option<AnyPearl> {
        let imposter = self.imposter.swap_remove(index)?;
        Some(AnyPearl { imposter })
    }
}
