use crate::{Handle, HandleMap, HandleResult};

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
    pub fn get_parent<'a>(&'a self, map: &'a NodeMap<ID>) -> Option<&Node<ID>> {
        let parent = self.parent.as_ref()?;
        map.get(parent).ok()
    }

    /// Gets the children for this node using the given `map`
    ///
    /// ## Warning
    /// Using a the wrong map for this node is ***undefined behaviour***, and may result in a panic
    pub fn get_children<'a>(&'a self, map: &'a NodeMap<ID>) -> HandleResult<Vec<&Node<ID>>> {
        let child_iter = self.children.iter();
        Ok(child_iter
            .filter_map(|h| map.get(h).ok())
            .collect::<Vec<_>>())
    }
}
