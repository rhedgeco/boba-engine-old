use std::marker::PhantomData;

use imposters::collections::vec::ImposterVec;

use crate::{Pearl, PearlId};

pub struct QueryItem<'a> {
    id: PearlId,
    data: Vec<&'a ImposterVec>,
}

impl<'a> QueryItem<'a> {
    #[inline]
    pub fn new<T: Pearl>() -> Self {
        Self {
            id: PearlId::of::<T>(),
            data: Vec::new(),
        }
    }

    #[inline]
    pub(super) fn add_vec(&mut self, vec: &'a ImposterVec) {
        self.data.push(vec)
    }

    #[inline]
    pub fn id(&self) -> PearlId {
        self.id
    }

    #[inline]
    pub fn iter<T: Pearl>(&'a self) -> Option<QueryIter<'a, T>> {
        if self.id != PearlId::of::<T>() {
            return None;
        }

        Some(QueryIter::new(self))
    }
}

pub struct QueryIter<'a, T: Pearl> {
    item_index: usize,
    vecs_index: usize,
    vecs: &'a Vec<&'a ImposterVec>,
    _type: PhantomData<&'a T>,
}

impl<'a, T: Pearl> QueryIter<'a, T> {
    fn new(query: &'a QueryItem<'a>) -> Self {
        Self {
            item_index: 0,
            vecs_index: 0,
            vecs: &query.data,
            _type: PhantomData,
        }
    }
}

impl<'a, T: Pearl> Iterator for QueryIter<'a, T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // loops until a valid pointer index is found, or we run out of vecs
        loop {
            // get the current vec
            // SAFETY: The vec index is validated after it is incremented
            let vec = unsafe { self.vecs.get_unchecked(self.vecs_index) };

            // get a pointer from the vec
            let Some(ptr) = vec.get_ptr(self.item_index) else {

                // if the vec has been depleted, reset the item index
                // and increment the vec index
                self.item_index = 0;
                self.vecs_index += 1;

                // if the vec index is invalid, return None
                if self.vecs_index >= self.vecs.len() {
                    return None;
                }
                
                // if the vec index is valid,
                // we can continue the loop with the new vec
                continue;
            };

            // convert the pointer, and increment the item index
            let ptr = ptr as *mut T;
            self.item_index += 1;
            return Some(ptr);
        }
    }
}
