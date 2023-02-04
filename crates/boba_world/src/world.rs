use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use indexmap::{IndexMap, IndexSet};

use crate::{BobaId, Handle, HandleMap};

/// A central location to store [`Node`] and [`Pearl`] obejcts
#[derive(Default)]
pub struct World {
    nodes: HandleMap<Node>,
    pearls: HandleMap<Box<dyn Any>>,
}

impl World {
    /// Creates a new world
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to a map of all nodes in this world
    pub fn nodes(&self) -> &HandleMap<Node> {
        &self.nodes
    }

    /// Returns a reference to the pearl associated with `link`.
    ///
    /// Returns `None` if the link is invalid.
    pub fn get_pearl<T: 'static>(&self, link: &PearlLink<T>) -> Option<&Pearl<T>> {
        let any = self.pearls.get(&link.handle)?;
        any.downcast_ref::<Pearl<T>>()
    }

    /// Returns a mutable reference to the pearl associated with `link`.
    ///
    /// Returns `None` if the link is invalid.
    pub fn get_pearl_mut<T: 'static>(&mut self, link: &PearlLink<T>) -> Option<&mut Pearl<T>> {
        let any = self.pearls.get_mut(&link.handle)?;
        any.downcast_mut::<Pearl<T>>()
    }

    /// Returns the pearl of type `T` associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid, or the pearl does not exist for this handle
    pub fn get_node_pearl<T: 'static>(&self, handle: &Handle<Node>) -> Option<&Pearl<T>> {
        let node = self.nodes.get(handle)?;
        let pearl_handle = node.pearls.get(&TypeId::of::<T>())?;
        let any = self.pearls.get(pearl_handle)?;
        any.downcast_ref::<Pearl<T>>()
    }

    /// Returns the pearl of type `T` associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid, or the pearl does not exist for this handle
    pub fn get_node_pearl_link<T: 'static>(&self, handle: &Handle<Node>) -> Option<PearlLink<T>> {
        let node = self.nodes.get(handle)?;
        node.new_pearl_link::<T>()
    }

    /// Returns the pearl of type `T` associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid, or the pearl does not exist for this handle
    pub fn get_node_pearl_mut<T: 'static>(
        &mut self,
        handle: &Handle<Node>,
    ) -> Option<&mut Pearl<T>> {
        let node = self.nodes.get(handle)?;
        let pearl_handle = node.pearls.get(&TypeId::of::<T>())?;
        let any = self.pearls.get_mut(pearl_handle)?;
        any.downcast_mut::<Pearl<T>>()
    }

    /// Returns a vector of references to all the pearls of type `T` connected to the children of `handle`
    pub fn get_pearls_in_children<T: 'static>(&self, handle: &Handle<Node>) -> Vec<&Pearl<T>> {
        let Some(node) = self.nodes.get(handle) else {
            return Vec::new();
        };

        node.children
            .iter()
            .filter_map(|child_handle| self.get_node_pearl::<T>(child_handle))
            .collect()
    }

    /// Calls a function on references to all the pearls of type `T` connected to the children of `handle`
    pub fn call_pearls_in_children<T: 'static>(
        &self,
        handle: &Handle<Node>,
        mut f: impl FnMut(&Pearl<T>),
    ) {
        let Some(node) = self.nodes.get(handle) else {
            return;
        };

        for child_handle in node.children.iter() {
            let Some(pearl) = self.get_node_pearl(child_handle) else { continue; };
            f(pearl);
        }
    }

    /// Calls a function on references to all the pearls of type `T` connected to the children of `handle`.
    ///
    /// This function also recurses all the way down each node in the heirarchy
    pub fn call_pearls_in_children_recursive<T: 'static>(
        &self,
        handle: &Handle<Node>,
        mut f: impl FnMut(&Pearl<T>),
    ) {
        fn recurse<T: 'static>(
            world: &World,
            handle: &Handle<Node>,
            f: &mut impl FnMut(&Pearl<T>),
        ) {
            let Some(node) = world.nodes.get(handle) else {
                return;
            };

            for child_handle in node.children.iter() {
                let Some(pearl) = world.get_node_pearl(child_handle) else { continue; };
                f(pearl);
                recurse(world, child_handle, f);
            }
        }

        recurse(self, handle, &mut f);
    }

    /// Returns a vector of mutable references all the pearls of type `T` connected to the children of `handle`
    pub fn get_pearls_in_children_mut<T: 'static>(
        &mut self,
        handle: &Handle<Node>,
    ) -> Vec<&mut Pearl<T>> {
        let Some(node) = self.nodes.get(handle) else {
            return Vec::new();
        };

        let links = node
            .children
            .iter()
            .filter_map(|child_handle| {
                let child = self.nodes.get(child_handle)?;
                child.pearls.get(&TypeId::of::<T>())
            })
            .collect::<IndexSet<&Handle<Box<dyn Any>>>>();

        self.pearls
            .get_many_mut(&links)
            .into_iter()
            .filter_map(|pearl| pearl.downcast_mut::<Pearl<T>>())
            .collect()
    }

    /// Calls a function on mutable references to all the pearls of type `T` connected to the children of `handle`
    pub fn call_pearls_in_children_mut<T: 'static>(
        &mut self,
        handle: &Handle<Node>,
        mut f: impl FnMut(&mut Pearl<T>),
    ) {
        let Some(node) = self.nodes.get(handle) else {
                return;
            };

        for child_handle in node.children.iter() {
            let Some(child) = self.nodes.get(child_handle) else { continue; };
            let Some(pearl_handle) = child.pearls.get(&TypeId::of::<T>()) else { continue; };
            let Some(any) = self.pearls.get_mut(pearl_handle) else { continue; };
            let Some(pearl) = any.downcast_mut::<Pearl<T>>() else { continue; };
            f(pearl);
        }
    }

    /// Calls a function on mutable references to all the pearls of type `T` connected to the children of `handle`.
    ///
    /// This function also recurses all the way down each node in the heirarchy
    pub fn call_pearls_in_children_mut_recursive<T: 'static>(
        &mut self,
        handle: &Handle<Node>,
        mut f: impl FnMut(&mut Pearl<T>),
    ) {
        fn recurse<T: 'static>(
            nodes: &HandleMap<Node>,
            pearls: &mut HandleMap<Box<dyn Any>>,
            handle: &Handle<Node>,
            f: &mut impl FnMut(&mut Pearl<T>),
        ) {
            let Some(node) = nodes.get(handle) else {
                return;
            };

            for child_handle in node.children.iter() {
                let Some(child) = nodes.get(child_handle) else { continue; };
                let Some(pearl_handle) = child.pearls.get(&TypeId::of::<T>()) else { continue; };
                let Some(any) = pearls.get_mut(pearl_handle) else { continue; };
                let Some(pearl) = any.downcast_mut::<Pearl<T>>() else { continue; };
                f(pearl);
                recurse(nodes, pearls, child_handle, f);
            }
        }

        recurse(&self.nodes, &mut self.pearls, handle, &mut f);
    }

    /// Inserts a pearl into a node and returns a link to the pearl
    ///
    /// Returns `None` if the `handle` is invalid
    pub fn insert_node_pearl<T: 'static>(
        &mut self,
        data: T,
        handle: &Handle<Node>,
    ) -> Option<PearlLink<T>> {
        let node = self.nodes.get_mut(handle)?;
        let pearl = Pearl::new(data, handle.clone());
        let pearl_handle = self.pearls.insert(Box::new(pearl));
        node.pearls.insert(TypeId::of::<T>(), pearl_handle.clone());
        Some(PearlLink::new(pearl_handle))
    }
}

#[derive(Default)]
pub struct Node {
    parent: Box<Option<Handle<Node>>>,
    children: IndexSet<Handle<Node>>,
    pearls: IndexMap<TypeId, Handle<Box<dyn Any>>>,
}

impl Node {
    /// Creates a new empty node
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to this nodes parent
    ///
    /// Will be `&None` if this node doesn't have a parent
    pub fn parent(&self) -> &Option<Handle<Node>> {
        &self.parent
    }

    /// Returns a reference to this nodes children
    pub fn children(&self) -> &IndexSet<Handle<Node>> {
        &self.children
    }

    /// Returns a link to the pearl of type `T` that is connected to this node
    ///
    /// Returns `None` if there is no pearl of type `T`
    pub fn new_pearl_link<T: 'static>(&self) -> Option<PearlLink<T>> {
        let handle = self.pearls.get(&TypeId::of::<T>())?;
        Some(PearlLink::new(handle.clone()))
    }
}

pub struct PearlLink<T: 'static> {
    handle: Handle<Box<dyn Any>>,
    _type: PhantomData<*const T>,
}

impl<T: 'static> Clone for PearlLink<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            _type: PhantomData,
        }
    }
}

impl<T: 'static> PearlLink<T> {
    fn new(handle: Handle<Box<dyn Any>>) -> Self {
        Self {
            handle,
            _type: PhantomData,
        }
    }
}

pub struct Pearl<T: 'static> {
    data: T,
    id: BobaId,
    node: Handle<Node>,
}

impl<T: 'static> Eq for Pearl<T> {}

impl<T: 'static> PartialEq for Pearl<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T: 'static> DerefMut for Pearl<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: 'static> Deref for Pearl<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: 'static> Pearl<T> {
    /// Creates a new pearl with `data` and `node`
    fn new(data: T, node: Handle<Node>) -> Self {
        Self {
            data,
            id: BobaId::new(),
            node,
        }
    }

    /// Returns the [`BobaId`] for this pearl
    pub fn id(&self) -> &BobaId {
        &self.id
    }

    /// Returns a handle to the node this pearl is connected to
    pub fn node(&self) -> &Handle<Node> {
        &self.node
    }
}
