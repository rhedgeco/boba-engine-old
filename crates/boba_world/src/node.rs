use std::any::{Any, TypeId};

use indexmap::IndexMap;

use crate::{Handle, HandleMap, Pearl, PearlHandle, World};

/// A map containing [`Node`] objects
pub type NodeMap<const ID: usize> = HandleMap<Node<ID>, ID>;

/// A handle to a [`Node`] object contained in some [`NodeMap`]
pub type NodeHandle<const ID: usize> = Handle<Node<ID>, ID>;

/// A node in a heirarchy of nodes
///
/// Makes use of [`NodeHandle`] to track parent child relationships
#[derive(Default)]
pub struct Node<const ID: usize> {
    parent: Option<NodeHandle<ID>>,
    children: Vec<NodeHandle<ID>>,
    pearls: IndexMap<TypeId, Box<dyn Any>>,
}

impl<const ID: usize> Node<ID> {
    /// Creates a new empty node
    pub fn new() -> Self {
        Self::default()
    }

    /// Flushes all invalid handles in this node
    ///
    /// Internally this just calls the following methods
    /// - `flush_parent()`
    /// - `flush_children()`
    pub fn flush_all(&mut self) {
        self.flush_parent();
        self.flush_children();
    }

    /// Resets the parent to `None` if it is invalid
    pub fn flush_parent(&mut self) {
        match &mut self.parent {
            Some(handle) if !handle.is_valid() => self.parent = None,
            _ => (),
        }
    }

    /// Removes all children that have invalid handles
    pub fn flush_children(&mut self) {
        self.children.retain(|h| h.is_valid());
    }

    /// Gets the parent for this node using the given `map`
    ///
    /// ## Warning
    /// Using a the wrong map for this node is ***undefined behaviour***, and may result in a panic
    pub fn get_parent<'a>(&'a self, world: &'a World<ID>) -> Option<&Node<ID>> {
        let parent = self.parent.as_ref()?;
        world.nodes.get(parent).ok()
    }

    /// Gets the children for this node using the given `map`
    ///
    /// ## Warning
    /// Using a the wrong map for this node is ***undefined behaviour***, and may result in a panic
    pub fn get_children<'a>(&'a self, world: &'a World<ID>) -> Vec<&Node<ID>> {
        let child_iter = self.children.iter();
        child_iter
            .filter_map(|h| world.nodes.get(h).ok())
            .collect::<Vec<_>>()
    }

    /// Gets a reference to the [`Pearl`] containing with `T` from this given node.
    /// Returns `None` if the pearl does not exist.
    ///
    /// ## Warning
    /// Using a the wrong map for this node is ***undefined behaviour***, and may result in a panic
    pub fn get_pearl<'a, T: 'static>(&'a self, world: &'a World<ID>) -> Option<&Pearl<T>> {
        let handle = self.pearls.get(&TypeId::of::<T>())?;
        let handle = handle.downcast_ref::<PearlHandle<T, ID>>().unwrap();
        world.pearls.get(handle).ok()
    }

    /// Gets a mutable reference to the [`Pearl`] containing with `T` from this given node.
    /// Returns `None` if the pearl does not exist.
    ///
    /// ## Warning
    /// Using a the wrong map for this node is ***undefined behaviour***, and may result in a panic
    pub fn get_pearl_mut<'a, T: 'static>(
        &'a self,
        world: &'a mut World<ID>,
    ) -> Option<&mut Pearl<T>> {
        let handle = self.pearls.get(&TypeId::of::<T>())?;
        let handle = handle.downcast_ref::<PearlHandle<T, ID>>().unwrap();
        world.pearls.get_mut(handle).ok()
    }
}
