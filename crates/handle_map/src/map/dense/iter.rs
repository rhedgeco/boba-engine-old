use std::slice::{Iter, IterMut};

use crate::Handle;

pub struct Data<'a, T> {
    pub(super) iter: Iter<'a, (Handle<T>, T)>,
}

impl<'a, T> Iterator for Data<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, data) = self.iter.next()?;
        Some(data)
    }
}

pub struct DataMut<'a, T> {
    pub(super) iter: IterMut<'a, (Handle<T>, T)>,
}

impl<'a, T> Iterator for DataMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, data) = self.iter.next()?;
        Some(data)
    }
}

pub struct Handles<'a, T> {
    pub(super) iter: Iter<'a, (Handle<T>, T)>,
}

impl<'a, T> Iterator for Handles<'a, T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (handle, _) = self.iter.next()?;
        Some(*handle)
    }
}
