use std::{
    any::{type_name, Any, TypeId},
    cell::{RefCell, RefMut},
    hash::Hash,
    mem::replace,
    rc::Rc,
};

use indexmap::{IndexMap, IndexSet};
use log::{error, warn};

use crate::{BobaId, BobaResult, Pearl};

struct NodeRelations {
    parent: Option<Node>,
    children: IndexSet<Node>,
}

struct InnerNode {
    id: BobaId,
    /// `relations` can only be accessed internally and has no exposing api.
    /// So any cloning of ref cell and multiple accesses to the node will not break
    /// any of the methods that utilize the relations.
    ///
    /// Any errors related to the relations field, is directly tied to bad logic in one of the nodes methods.
    relations: RefCell<NodeRelations>,
    pearls: RefCell<IndexMap<TypeId, Box<dyn Any>>>,
}

/// A node in a hierarchy of nodes.
///
/// This will represent a node in a branching list of nodes.
/// The list is doubly linked, and logic flow may move up and down the branches as necessary
#[derive(Clone)]
pub struct Node {
    inner: Rc<InnerNode>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.id.hash(state);
    }
}

impl Node {
    /// Gets the [`BobaId`] for this node
    pub fn id(&self) -> &BobaId {
        &self.inner.id
    }

    /// Gets clone of the current nodes parent, returning `None` if no parent exists.
    ///
    /// This is not free as it has to make a clone, and should be cached if the parent is to be used multiple times.
    pub fn get_parent(&self) -> Option<Node> {
        self.inner.relations.borrow().parent.clone()
    }

    /// Gets a vector of clones of the current nodes children.
    ///
    /// This is not free as it has to clone every child into a new vector. This should be cached if it has to be used multiple times.
    pub fn get_children(&self) -> Vec<Node> {
        let children = &self.inner.relations.borrow().children;
        children.iter().cloned().collect()
    }

    /// Creates a new pearl out of `pearl_data` and adds it to the node
    pub fn add_pearl<T: 'static>(&self, pearl_data: T) -> Pearl<T> {
        let pearl = Pearl::new(pearl_data, self.clone());
        let mut pearl_map = self.inner.pearls.borrow_mut();
        pearl_map.insert(TypeId::of::<T>(), Box::new(pearl.clone()));
        pearl
    }

    /// Queries the nodes pearls, and returns a clone of the pearl if it exists
    pub fn get_pearl<T: 'static>(&self) -> Option<Pearl<T>> {
        let pearl_map = self.inner.pearls.borrow();
        let pearl = pearl_map.get(&TypeId::of::<T>())?;
        Some(pearl.downcast_ref::<Pearl<T>>().unwrap().clone())
    }

    /// Queries this node and all its children, and gets a clone of every matching pearl of type `T`
    pub fn get_pearls_in_children<T: 'static>(&self) -> Vec<Pearl<T>> {
        fn recurse_children<T: 'static>(node: &Node, pearls: &mut Vec<Pearl<T>>) {
            if let Some(pearl) = node.get_pearl::<T>() {
                pearls.push(pearl);
            }

            let relation = node.inner.relations.borrow();
            for child in relation.children.iter() {
                recurse_children(child, pearls);
            }
        }

        let mut pearls = Vec::<Pearl<T>>::new();
        recurse_children(self, &mut pearls);
        pearls
    }

    /// Queries this node and all its children for pearls of type `T`, and calls the function `f` on them
    ///
    /// This is faster than trying to do the same using `get_pearls_in_children`
    /// because a new `Vec` is not allocated/updated and the pearls do not have to be cloned.
    pub fn call_pearls_in_children<T: 'static>(&self, f: impl Fn(&Pearl<T>) -> BobaResult) {
        fn recurse_children<T: 'static>(node: &Node, f: &impl Fn(&Pearl<T>) -> BobaResult) {
            let pearl_map = node.inner.pearls.borrow();
            if let Some(any_pearl) = pearl_map.get(&TypeId::of::<T>()) {
                let pearl = any_pearl.downcast_ref::<Pearl<T>>().unwrap();
                if let Err(e) = f(pearl) {
                    let name = type_name::<T>();
                    error!("There was an error while calling Pearl<{name}> using 'call_pearls_in_children'. Error: {e}")
                }
            }

            let relation = node.inner.relations.borrow();
            for child in relation.children.iter() {
                recurse_children::<T>(child, f);
            }
        }

        recurse_children::<T>(self, &f);
    }

    /// Sets the parent of this node to `parent`.
    ///
    /// This is not a simple operation and it has to do a recursive check up the tree of nodes to check for cyclic parent structures.
    /// Any cyclic parent structures will be auto-magically resolved by reordering the stack of nodes in the best way possible.
    ///
    /// The complexity of this operation is **O(n)** where n is the depth of the parent node from the root.
    /// At the end of the day, it will still be very fast, but it is good to know this if many parenting operations are required.
    pub fn set_parent(&self, parent: &Node) {
        // check to make sure nodes are not identical
        if parent == self {
            warn!("Tried to set node parent as itself. Skipping parent operation.");
            return;
        }

        // get current nodes relations
        let mut this_relation = self.inner.relations.borrow_mut();

        // check if the current node has a parent
        if let Some(this_parent) = &this_relation.parent {
            // if it does, check to make sure the parent layout is not already correct
            if parent == this_parent {
                warn!("Tried to set parent node but it is already the current parent. Skipping parent operation.");
                return;
            }

            // remove this node from its parents set of linked children
            let mut this_parent_relation = this_parent.inner.relations.borrow_mut();
            this_parent_relation.children.remove(self);
        }

        // create recursive function to check for cyclic parent structure
        fn is_cyclic(current_node: &Node, next_parent_relation: &RefMut<NodeRelations>) -> bool {
            match &next_parent_relation.parent {
                None => false,
                Some(next_parent) => {
                    if current_node == next_parent {
                        true
                    } else {
                        let next_parent_relation = next_parent.inner.relations.borrow_mut();
                        is_cyclic(current_node, &next_parent_relation)
                    }
                }
            }
        }

        // get new parent relations
        let mut new_parent_relation = parent.inner.relations.borrow_mut();

        // check if this parent relation is cyclic
        // if it is, reorder the stack to resolve cyclic dependency
        if is_cyclic(self, &new_parent_relation) {
            warn!(
                "Attemted to create cyclic parent structure. Reordered parent stack to avoid this."
            );
            // remove the new parent node from its parents set of children
            if let Some(new_parents_parent) = &new_parent_relation.parent {
                // check for case where the new parents parent is actually the current node
                // holy canoli this gets wacky. I hope to god no one will have to read this ever again
                // and all the unit tests I could think of just prove that everything works without modification
                if new_parents_parent == self {
                    this_relation.children.remove(parent);
                } else {
                    let mut new_parents_parents_relation =
                        new_parents_parent.inner.relations.borrow_mut();
                    new_parents_parents_relation.children.remove(parent);
                };
            }

            // set the new parents parent as this nodes parent
            let _ = replace(
                &mut new_parent_relation.parent,
                this_relation.parent.clone(),
            );

            // add the new parent node as a child of this nodes old parent
            if let Some(this_nodes_old_parent) = &this_relation.parent {
                let mut this_nodes_old_parent_relation =
                    this_nodes_old_parent.inner.relations.borrow_mut();
                this_nodes_old_parent_relation
                    .children
                    .insert(parent.clone());
            }
        }

        // replace this nodes parent with the new one
        let _ = replace(&mut this_relation.parent, Some(parent.clone()));

        // add this node to the new parents child listrelations
        new_parent_relation.children.insert(self.clone());
    }
}

// TODO: Test parent system
#[cfg(test)]
mod tests {
    use super::*;

    fn create_node() -> Node {
        let relations = NodeRelations {
            parent: None,
            children: IndexSet::new(),
        };

        let inner = InnerNode {
            id: BobaId::new(),
            relations: RefCell::new(relations),
            pearls: RefCell::new(IndexMap::new()),
        };

        Node {
            inner: Rc::new(inner),
        }
    }

    #[test]
    fn self_assign() {
        let node = create_node();
        node.set_parent(&node);

        let relation = node.inner.relations.borrow();
        assert!(relation.parent.is_none());
        assert!(relation.children.len() == 0);
    }

    #[test]
    fn redundant_assign() {
        let node1 = create_node();
        let node2 = create_node();
        node2.set_parent(&node1);
        node2.set_parent(&node1);

        let relation1 = node1.inner.relations.borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        drop(relation1);

        let relation2 = node2.inner.relations.borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.len() == 0);
    }

    #[test]
    fn simple_chain() {
        let node1 = create_node();
        let node2 = create_node();
        let node3 = create_node();
        node2.set_parent(&node1);
        node3.set_parent(&node2);

        let relation1 = node1.inner.relations.borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        drop(relation1);

        let relation2 = node2.inner.relations.borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.contains(&node3));
        drop(relation2);

        let relation3 = node3.inner.relations.borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node2);
        assert!(relation3.children.len() == 0);
    }

    #[test]
    fn branching_chain() {
        let node1 = create_node();
        let node2 = create_node();
        let node3 = create_node();
        let node4 = create_node();
        let node5 = create_node();
        node2.set_parent(&node1);
        node3.set_parent(&node1);
        node4.set_parent(&node2);
        node5.set_parent(&node2);

        let relation1 = node1.inner.relations.borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        assert!(relation1.children.contains(&node3));
        drop(relation1);

        let relation2 = node2.inner.relations.borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.contains(&node4));
        assert!(relation2.children.contains(&node5));
        drop(relation2);

        let relation3 = node3.inner.relations.borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node1);
        assert!(relation3.children.len() == 0);
        drop(relation3);

        let relation4 = node4.inner.relations.borrow();
        assert!(relation4.parent.as_ref().unwrap() == &node2);
        assert!(relation4.children.len() == 0);
        drop(relation4);

        let relation5 = node5.inner.relations.borrow();
        assert!(relation5.parent.as_ref().unwrap() == &node2);
        assert!(relation5.children.len() == 0);
    }

    #[test]
    fn simple_reassign() {
        let node1 = create_node();
        let node2 = create_node();
        let node3 = create_node();
        node3.set_parent(&node1);
        node3.set_parent(&node2);

        let relation1 = node1.inner.relations.borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.len() == 0);
        drop(relation1);

        let relation2 = node2.inner.relations.borrow();
        assert!(relation2.parent.is_none());
        assert!(relation2.children.contains(&node3));
        drop(relation2);

        let relation3 = node3.inner.relations.borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node2);
        assert!(relation3.children.len() == 0);
    }

    // first structure is made, and then the 3 is reparented to the 5
    // this tests the cyclic resolving feature as well
    //
    //   1              1
    //  / \            / \
    // 2   3    ->    2   5
    //    / \              \
    //   4   5              3
    //                     /
    //                    4
    #[test]
    fn branching_reassign() {
        let node1 = create_node();
        let node2 = create_node();
        let node3 = create_node();
        let node4 = create_node();
        let node5 = create_node();
        node2.set_parent(&node1);
        node3.set_parent(&node1);
        node4.set_parent(&node3);
        node5.set_parent(&node3);
        node3.set_parent(&node5);

        let relation1 = node1.inner.relations.borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        assert!(relation1.children.contains(&node5));
        drop(relation1);

        let relation2 = node2.inner.relations.borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.len() == 0);
        drop(relation2);

        let relation3 = node3.inner.relations.borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node5);
        assert!(relation3.children.contains(&node4));
        drop(relation3);

        let relation4 = node4.inner.relations.borrow();
        assert!(relation4.parent.as_ref().unwrap() == &node3);
        assert!(relation4.children.len() == 0);
        drop(relation4);

        let relation5 = node5.inner.relations.borrow();
        assert!(relation5.parent.as_ref().unwrap() == &node1);
        assert!(relation5.children.contains(&node3));
    }
}
