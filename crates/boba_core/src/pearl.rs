use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    ops::DerefMut,
    rc::Rc,
    sync::atomic::AtomicU64,
};

use thiserror::Error;

use crate::{BobaResources, BobaResult, BobaStage, StageRegistrar};

/// The Id for a Pearl
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PearlId {
    _id: u64,
}

impl PearlId {
    /// Creates a new PearlId.
    ///
    /// It increments a atomic u64 and uses that as its id value, so each Id will be constructed with a unique value.
    /// This will never run out because there are more ids than there are atoms in the universe.
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            _id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

/// An error returned by [`Pearl::destroy`].
#[derive(Debug, Error)]
#[error("Pearl cannot be destroyed. Error: {0}")]
pub struct PearlDestroyError(BorrowMutError);

/// An error returned by [`Pearl::borrow`].
#[derive(Debug, Error)]
pub enum PearlError {
    #[error("Pearl has been destroyed")]
    Destroyed,
    #[error("Pearl cannot be borrowed. Error: {0}")]
    Borrowed(BorrowError),
}

/// An error returned by [`Pearl::borrow_mut`].
#[derive(Debug, Error)]
pub enum PearlMutError {
    #[error("Pearl has been destroyed")]
    Destroyed,
    #[error("Pearl cannot be borrowed as mutable. Error: {0}")]
    Borrowed(BorrowMutError),
}

/// The core data management object in BobaEngine.
///
/// It is useful for multiple objects to hold references to the same struct.
pub struct Pearl<T>
where
    T: RegisterStages,
{
    id: PearlId,
    data: Rc<RefCell<Option<T>>>,
}

impl<T> Clone for Pearl<T>
where
    T: RegisterStages,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T> Pearl<T>
where
    T: RegisterStages,
{
    /// Gets the unique id of the current pearl
    pub fn id(&self) -> &PearlId {
        &self.id
    }

    /// Destroys the current pearl.
    ///
    /// Can fail if the pearl is currently being borrowed somewhere else.
    pub fn destroy(&self) -> Result<(), PearlDestroyError> {
        let mut borrow = match self.data.as_ref().try_borrow_mut() {
            Ok(borrow) => borrow,
            Err(e) => return Err(PearlDestroyError(e)),
        };

        drop(std::mem::replace(borrow.deref_mut(), None));

        Ok(())
    }

    /// Gets the contents of the pearl as an immutable reference.
    ///
    /// Can fail if the pearl is either already destroyed, or the pearl is already mutably borrowed.
    pub fn borrow(&self) -> Result<Ref<T>, PearlError> {
        let borrow = match self.data.as_ref().try_borrow() {
            Ok(borrow) => borrow,
            Err(e) => return Err(PearlError::Borrowed(e)),
        };

        if borrow.as_ref().is_none() {
            return Err(PearlError::Destroyed);
        };

        Ok(Ref::map(borrow, |data| data.as_ref().unwrap()))
    }

    /// Gets the contents of the pearl as an mutable reference.
    ///
    /// Can fail if the pearl is either already destroyed, or the pearl is already borrowed.
    pub fn borrow_mut(&self) -> Result<RefMut<T>, PearlMutError> {
        let borrow = match self.data.as_ref().try_borrow_mut() {
            Ok(borrow) => borrow,
            Err(e) => return Err(PearlMutError::Borrowed(e)),
        };

        if borrow.as_ref().is_none() {
            return Err(PearlMutError::Destroyed);
        };

        Ok(RefMut::map(borrow, |data| data.as_mut().unwrap()))
    }
}

/// Used to wrap an object in a new Pearl
pub trait WrapPearl<T>
where
    T: RegisterStages,
{
    fn wrap_pearl(self) -> Pearl<T>;
}

impl<T> WrapPearl<T> for T
where
    T: RegisterStages,
{
    fn wrap_pearl(self) -> Pearl<T> {
        Pearl {
            id: PearlId::new(),
            data: Rc::new(RefCell::new(Some(self))),
        }
    }
}

/// Base trait for being able to register stages with the boba system
pub trait RegisterStages: 'static
where
    Self: Sized,
{
    fn register(pearl: &Pearl<Self>, stages: &mut impl StageRegistrar);
}

pub trait PearlStage<Stage>: RegisterStages
where
    Stage: BobaStage,
{
    fn update(&mut self, data: &Stage::Data, resources: &mut BobaResources) -> BobaResult;
}
