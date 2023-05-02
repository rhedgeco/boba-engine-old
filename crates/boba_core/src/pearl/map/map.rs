use std::{
    any::Any,
    collections::hash_map::Entry,
    ops::{AddAssign, SubAssign},
    slice::{Iter, IterMut},
};

use fxhash::FxHashMap;

use crate::pearl::{Pearl, PearlExt, PearlId, PearlProvider};

use super::{Handle, InsertDestroyQueue, PearlData, PearlMapAccess, PearlMapQueue, RawHandle};

pub(super) struct HandleRoute {
    pub handle: RawHandle,
    pub index: Option<usize>,
}

impl HandleRoute {
    pub fn new(handle: RawHandle, index: usize) -> Self {
        Self {
            handle,
            index: Some(index),
        }
    }
}

pub(super) struct PearlVec {
    pearls: Box<dyn Any>,
}

impl PearlVec {
    pub fn new<P: Pearl>() -> Self {
        Self {
            pearls: Box::new(Vec::<PearlData<P>>::new()),
        }
    }

    #[inline]
    pub fn downcast_ref<P: Pearl>(&self) -> Option<&Vec<PearlData<P>>> {
        self.pearls.downcast_ref::<Vec<PearlData<P>>>()
    }

    #[inline]
    pub fn downcast_mut<P: Pearl>(&mut self) -> Option<&mut Vec<PearlData<P>>> {
        self.pearls.downcast_mut::<Vec<PearlData<P>>>()
    }
}

#[derive(Default)]
pub struct PearlMap {
    map_indices: FxHashMap<PearlId, u16>,
    handle_routes: Vec<Vec<HandleRoute>>,
    open_routes: Vec<Vec<RawHandle>>,
    pearl_maps: Vec<PearlVec>,
    map_length: Vec<u32>,
}

impl PearlProvider for PearlMap {
    fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.get(handle)
    }

    fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.get_mut(handle)
    }
}

impl PearlMap {
    pub fn insert<P: Pearl>(&mut self, pearl: P, on_new_map: impl FnOnce()) -> Handle<P> {
        let map_index = self.get_map_index::<P>(on_new_map) as usize;
        let routes = &mut self.handle_routes[map_index];
        let open = &mut self.open_routes[map_index];
        let pearl_map = self.pearl_maps[map_index].downcast_mut::<P>().unwrap();

        self.map_length[map_index].add_assign(1);
        match open.pop() {
            Some(open_handle) => {
                let route = &mut routes[open_handle.uindex()];
                route.index = Some(pearl_map.len());
                let handle = route.handle.into_type();
                pearl_map.push(PearlData::new(pearl, handle));
                P::on_insert(handle, self);
                handle
            }
            None => {
                let handle = RawHandle::from_raw_usize_parts(routes.len(), map_index, 0);
                routes.push(HandleRoute::new(handle, pearl_map.len()));
                pearl_map.push(PearlData::new(pearl, handle.into_type()));
                P::on_insert(handle.into_type(), self);
                handle.into_type()
            }
        }
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        let routes = self.handle_routes.get_mut(handle.umap_id())?;
        let route = routes.get_mut(handle.uindex())?;
        if route.handle != handle {
            return None;
        }

        let pearl_index = std::mem::replace(&mut route.index, None)?;
        route.handle.increment_generation();
        self.open_routes[handle.umap_id()].push(route.handle);

        let pearl_map = self.pearl_maps[handle.umap_id()].downcast_mut::<P>()?;
        let mut pearl_data = pearl_map.swap_remove(pearl_index);
        if let Some(swapped_data) = pearl_map.get_mut(pearl_index) {
            routes[swapped_data.handle().uindex()].index = Some(pearl_index);
        }

        self.map_length[handle.umap_id()].sub_assign(1);
        P::on_remove(&mut pearl_data, self);
        Some(pearl_data.into_data().0)
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        let pearl_index = self.get_pearl_index(handle)?;
        let pearl_map = self.pearl_maps[handle.umap_id()].downcast_ref::<P>()?;
        Some(&pearl_map[pearl_index])
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        let pearl_index = self.get_pearl_index(handle)?;
        let pearl_map = self.pearl_maps[handle.umap_id()].downcast_mut::<P>()?;
        Some(&mut pearl_map[pearl_index])
    }

    pub fn iter<P: Pearl>(&self) -> Iter<PearlData<P>> {
        let Some(map_index) = self.map_indices.get(&P::id()) else {
            return [].iter();
        };

        let Some(pearl_map) = self.pearl_maps[*map_index as usize].downcast_ref::<P>() else {
            return [].iter();
        };

        pearl_map.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> IterMut<PearlData<P>> {
        let Some(map_index) = self.map_indices.get(&P::id()) else {
            return [].iter_mut();
        };

        let Some(pearl_map) = self.pearl_maps[*map_index as usize].downcast_mut::<P>() else {
            return [].iter_mut();
        };

        pearl_map.iter_mut()
    }

    pub fn as_access(&mut self) -> PearlMapAccess {
        PearlMapAccess::new(&self.map_indices, &self.handle_routes, &mut self.pearl_maps)
    }

    pub fn as_queue<'a, 'b: 'a, T: InsertDestroyQueue>(
        &'a self,
        queue: &'b mut T,
    ) -> PearlMapQueue<'a, T> {
        PearlMapQueue::new(
            &self.map_indices,
            &self.open_routes,
            &self.map_length,
            queue,
        )
    }

    pub fn split_access_queue<'a, 'b: 'a, T: InsertDestroyQueue>(
        &'a mut self,
        queue: &'b mut T,
    ) -> (PearlMapAccess<'a>, PearlMapQueue<'a, T>) {
        (
            PearlMapAccess::new(&self.map_indices, &self.handle_routes, &mut self.pearl_maps),
            PearlMapQueue::new(
                &self.map_indices,
                &self.open_routes,
                &self.map_length,
                queue,
            ),
        )
    }

    fn get_pearl_index(&self, handle: Handle<impl Pearl>) -> Option<usize> {
        let routes = self.handle_routes.get(handle.umap_id())?;
        let route = routes.get(handle.uindex())?;
        match route.handle == handle {
            true => route.index,
            false => None,
        }
    }

    fn get_map_index<P: Pearl>(&mut self, on_new_map: impl FnOnce()) -> u16 {
        match self.map_indices.entry(P::id()) {
            Entry::Occupied(e) => *e.into_mut(),
            Entry::Vacant(e) => {
                let map_id = u16::try_from(self.map_length.len())
                    .expect("PearlMap type capacity overflow. Pearl types are over u16::MAX");
                self.handle_routes.push(Vec::new());
                self.open_routes.push(Vec::new());
                self.pearl_maps.push(PearlVec::new::<P>());
                self.map_length.push(0);
                let map_id = *e.insert(map_id);
                on_new_map();
                map_id
            }
        }
    }
}
