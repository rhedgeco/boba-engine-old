use std::{slice, vec};

use crate::Handle;

pub struct Iter<'a, T> {
    pub(super) iter: slice::Iter<'a, (Handle<T>, Option<T>)>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            let Some(data) = &next.1 else { continue };
            return Some((next.0, data));
        }
    }
}

pub struct IterMut<'a, T> {
    pub(super) iter: slice::IterMut<'a, (Handle<T>, Option<T>)>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Handle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            let Some(data) = &mut next.1 else { continue };
            return Some((next.0, data));
        }
    }
}

pub struct IntoIter<T> {
    pub(super) iter: vec::IntoIter<(Handle<T>, Option<T>)>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = (Handle<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            let Some(data) = next.1 else { continue };
            return Some((next.0, data));
        }
    }
}

pub struct Data<'a, T> {
    pub(super) iter: Iter<'a, T>,
}

impl<'a, T> Iterator for Data<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some(next.1)
    }
}

pub struct DataMut<'a, T> {
    pub(super) iter: IterMut<'a, T>,
}

impl<'a, T> Iterator for DataMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some(next.1)
    }
}

pub struct Handles<'a, T> {
    pub(super) iter: Iter<'a, T>,
}

impl<'a, T> Iterator for Handles<'a, T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some(next.0)
    }
}
