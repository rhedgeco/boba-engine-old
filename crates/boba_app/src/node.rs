use std::{
    cell::{RefCell, RefMut},
    hash::Hash,
    mem::replace,
    rc::Rc,
};

use hashbrown::HashSet;
use log::warn;

use crate::BobaId;

struct NodeRelations {
    parent: Option<Node>,
    children: HashSet<Node>,
}

#[derive(Clone)]
pub struct Node {
    id: BobaId,
    /// `relations` can only be accessed internally and has no exposing api.
    /// So any cloning of ref cell and multiple accesses to the node will not break
    /// any of the methods that utilize the relations.
    ///
    /// Any errors related to the relations field, is directly tied to bad logic in one of the nodes methods.
    relations: Rc<RefCell<NodeRelations>>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Self {
        let relations = NodeRelations {
            parent: None,
            children: HashSet::new(),
        };

        Self {
            id: BobaId::new(),
            relations: Rc::new(RefCell::new(relations)),
        }
    }

    pub fn set_parent(&self, parent: &Node) {
        // check to make sure nodes are not identical
        if parent == self {
            warn!("Tried to set node parent as itself. Skipping parent operation.");
            return;
        }

        // get current nodes relations
        let mut this_relation = self.relations.as_ref().borrow_mut();

        // check if the current node has a parent
        if let Some(this_parent) = &this_relation.parent {
            // if it does, check to make sure the parent layout is not already correct
            if parent == this_parent {
                warn!("Tried to set parent node but it is already the current parent. Skipping parent operation.");
                return;
            }

            // remove this node from its parents set of linked children
            let mut this_parent_relation = this_parent.relations.as_ref().borrow_mut();
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
                        let next_parent_relation = next_parent.relations.as_ref().borrow_mut();
                        is_cyclic(current_node, &next_parent_relation)
                    }
                }
            }
        }

        // get new parent relations
        let mut new_parent_relation = parent.relations.as_ref().borrow_mut();

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
                        new_parents_parent.relations.as_ref().borrow_mut();
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
                    this_nodes_old_parent.relations.as_ref().borrow_mut();
                this_nodes_old_parent_relation
                    .children
                    .insert(parent.clone());
            }
        }

        // replace this nodes parent with the new one
        let _ = replace(&mut this_relation.parent, Some(parent.clone()));

        // add this node to the new parents child list
        new_parent_relation.children.insert(self.clone());
    }
}

// TODO: Test parent system
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_assign() {
        let node = Node::new();
        node.set_parent(&node);

        let relation = node.relations.as_ref().borrow();
        assert!(relation.parent.is_none());
        assert!(relation.children.len() == 0);
    }

    #[test]
    fn redundant_assign() {
        let node1 = Node::new();
        let node2 = Node::new();
        node2.set_parent(&node1);
        node2.set_parent(&node1);

        let relation1 = node1.relations.as_ref().borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        drop(relation1);

        let relation2 = node2.relations.as_ref().borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.len() == 0);
    }

    #[test]
    fn simple_chain() {
        let node1 = Node::new();
        let node2 = Node::new();
        let node3 = Node::new();
        node2.set_parent(&node1);
        node3.set_parent(&node2);

        let relation1 = node1.relations.as_ref().borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        drop(relation1);

        let relation2 = node2.relations.as_ref().borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.contains(&node3));
        drop(relation2);

        let relation3 = node3.relations.as_ref().borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node2);
        assert!(relation3.children.len() == 0);
    }

    #[test]
    fn branching_chain() {
        let node1 = Node::new();
        let node2 = Node::new();
        let node3 = Node::new();
        let node4 = Node::new();
        let node5 = Node::new();
        node2.set_parent(&node1);
        node3.set_parent(&node1);
        node4.set_parent(&node2);
        node5.set_parent(&node2);

        let relation1 = node1.relations.as_ref().borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        assert!(relation1.children.contains(&node3));
        drop(relation1);

        let relation2 = node2.relations.as_ref().borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.contains(&node4));
        assert!(relation2.children.contains(&node5));
        drop(relation2);

        let relation3 = node3.relations.as_ref().borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node1);
        assert!(relation3.children.len() == 0);
        drop(relation3);

        let relation4 = node4.relations.as_ref().borrow();
        assert!(relation4.parent.as_ref().unwrap() == &node2);
        assert!(relation4.children.len() == 0);
        drop(relation4);

        let relation5 = node5.relations.as_ref().borrow();
        assert!(relation5.parent.as_ref().unwrap() == &node2);
        assert!(relation5.children.len() == 0);
    }

    #[test]
    fn simple_reassign() {
        let node1 = Node::new();
        let node2 = Node::new();
        let node3 = Node::new();
        node3.set_parent(&node1);
        node3.set_parent(&node2);

        let relation1 = node1.relations.as_ref().borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.len() == 0);
        drop(relation1);

        let relation2 = node2.relations.as_ref().borrow();
        assert!(relation2.parent.is_none());
        assert!(relation2.children.contains(&node3));
        drop(relation2);

        let relation3 = node3.relations.as_ref().borrow();
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
        let node1 = Node::new();
        let node2 = Node::new();
        let node3 = Node::new();
        let node4 = Node::new();
        let node5 = Node::new();
        node2.set_parent(&node1);
        node3.set_parent(&node1);
        node4.set_parent(&node3);
        node5.set_parent(&node3);
        node3.set_parent(&node5);

        let relation1 = node1.relations.as_ref().borrow();
        assert!(relation1.parent.is_none());
        assert!(relation1.children.contains(&node2));
        assert!(relation1.children.contains(&node5));
        drop(relation1);

        let relation2 = node2.relations.as_ref().borrow();
        assert!(relation2.parent.as_ref().unwrap() == &node1);
        assert!(relation2.children.len() == 0);
        drop(relation2);

        let relation3 = node3.relations.as_ref().borrow();
        assert!(relation3.parent.as_ref().unwrap() == &node5);
        assert!(relation3.children.contains(&node4));
        drop(relation3);

        let relation4 = node4.relations.as_ref().borrow();
        assert!(relation4.parent.as_ref().unwrap() == &node3);
        assert!(relation4.children.len() == 0);
        drop(relation4);

        let relation5 = node5.relations.as_ref().borrow();
        assert!(relation5.parent.as_ref().unwrap() == &node1);
        assert!(relation5.children.contains(&node3));
    }
}
