use std::slice::{Iter, IterMut};

use fxhash::FxHashMap;

use crate::pearl::{Pearl, PearlExt, PearlId};

use super::{ExclusiveStream, Handle, HandleRoute, PearlData, PearlVec};

pub struct PearlMapAccess<'a> {
    pub(super) map_indices: &'a FxHashMap<PearlId, u16>,
    handle_routes: &'a Vec<Vec<HandleRoute>>,
    pearl_maps: &'a mut Vec<PearlVec>,
}

impl<'a> PearlMapAccess<'a> {
    pub(super) fn new(
        map_indices: &'a FxHashMap<PearlId, u16>,
        handle_routes: &'a Vec<Vec<HandleRoute>>,
        pearl_maps: &'a mut Vec<PearlVec>,
    ) -> Self {
        Self {
            map_indices,
            handle_routes,
            pearl_maps,
        }
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

    pub fn exclusive_stream<P: Pearl>(&'a mut self) -> ExclusiveStream<'a, P> {
        ExclusiveStream::new(self)
    }

    fn get_pearl_index(&self, handle: Handle<impl Pearl>) -> Option<usize> {
        let routes = self.handle_routes.get(handle.umap_id())?;
        let route = routes.get(handle.uindex())?;
        match route.handle == handle {
            true => route.index,
            false => None,
        }
    }
}
