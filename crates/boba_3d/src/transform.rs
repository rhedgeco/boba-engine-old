use boba_core::pearls::{
    map::{Handle, PearlData, PearlProvider},
    Pearl,
};
use glam::{Mat4, Quat, Vec3};
use indexmap::IndexSet;
use log::warn;

pub struct TransformData {
    pub pos: Vec3,
    pub rot: Quat,
    pub scale: Vec3,
    pub parent: Option<Handle<Transform>>,
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            rot: Quat::IDENTITY,
            scale: Vec3::ONE,
            parent: None,
        }
    }
}

/// A structure for calculating location, rotation, and scale in a 3d space.
pub struct Transform {
    local_pos: Vec3,
    local_rot: Quat,
    local_scale: Vec3,
    parent_mat: Mat4,

    parent: Option<Handle<Self>>,
    children: IndexSet<Handle<Self>>,
}

impl Pearl for Transform {
    fn on_insert(handle: Handle<Self>, pearls: &mut impl PearlProvider) {
        let Some(pearl) = pearls.get(handle) else { return }; // ensure handle is valid
        let Some(parent_handle) = pearl.parent else { return }; // check if transform has a parent
        let Some(parent) = pearls.get_mut(parent_handle) else {
            // set this parent to none if a parent does not exist
            pearls.get_mut(handle).unwrap().parent = None;
            return;
        };

        // if this transform had a parent and the parent exists
        // add this handle to the parents child set
        parent.children.insert(handle);
        let parent_mat = parent.calculate_world_mat();
        pearls.get_mut(handle).unwrap().parent_mat = parent_mat;
    }

    fn on_remove(pearl: &mut PearlData<Self>, pearls: &mut impl PearlProvider) {
        match pearl.parent {
            Some(parent_handle) => {
                if let Some(parent) = pearls.get_mut(parent_handle) {
                    let world_mat = parent.calculate_world_mat();
                    parent.children.remove(&pearl.handle());
                    for child_handle in pearl.children.iter() {
                        if let Some(child) = pearls.get_mut(*child_handle) {
                            child.parent = Some(parent_handle);
                            child.parent_mat = world_mat;
                        }
                    }
                } else {
                    for child_handle in pearl.children.iter() {
                        if let Some(child) = pearls.get_mut(*child_handle) {
                            child.parent = None;
                        }
                    }
                }
            }
            None => {
                for child_handle in pearl.children.iter() {
                    if let Some(child) = pearls.get_mut(*child_handle) {
                        child.parent = None;
                    }
                }
            }
        }
    }
}

impl Transform {
    /// Returns a new transform with local `pos`, `rot`, and `scale`.
    pub fn new(data: TransformData) -> Self {
        Self {
            local_pos: data.pos,
            local_rot: data.rot,
            local_scale: data.scale,
            parent_mat: Mat4::IDENTITY,

            parent: data.parent,
            children: Default::default(),
        }
    }

    /// Calculates and returns the world position of this transform
    pub fn calculate_world_pos(&self) -> Vec3 {
        self.calculate_world_mat().to_scale_rotation_translation().2
    }

    /// Calculates and returns the world rotation of this transform
    pub fn calculate_world_rot(&self) -> Quat {
        self.calculate_world_mat().to_scale_rotation_translation().1
    }

    /// Calculates and returns the lossy world scale for this transform
    ///
    /// This will often be correct, but precision can be lost when the transform
    /// is a child of other transforms with certain rotations and scales.
    /// This is a limitation of forcing the child scale representaton into a Vec3.
    /// However, the world matrix will still contain accurate information.
    pub fn calculate_lossy_scale(&self) -> Vec3 {
        self.calculate_world_mat().to_scale_rotation_translation().0
    }

    /// Returns the local position of the transform relative to its parent.
    pub fn local_pos(&self) -> Vec3 {
        self.local_pos
    }

    /// Returns the local rotation of the transform relative to its parent.
    pub fn local_rot(&self) -> Quat {
        self.local_rot
    }

    /// Returns the local scale of the transform relative to its parent.
    pub fn local_scale(&self) -> Vec3 {
        self.local_scale
    }

    /// Calculates and returns the world matrix for this transform
    pub fn calculate_world_mat(&self) -> Mat4 {
        self.parent_mat * self.calculate_local_mat()
    }

    /// Calculates and returns the local matrix for this transform
    pub fn calculate_local_mat(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.local_scale, self.local_rot, self.local_pos)
    }
}

pub trait TransformHandleExt {
    fn set_local_pos(self, pos: Vec3, pearls: &mut impl PearlProvider);
    fn set_local_pos_no_sync(self, pos: Vec3, pearls: &mut impl PearlProvider);

    fn set_local_rot(self, rot: Quat, pearls: &mut impl PearlProvider);
    fn set_local_rot_no_sync(self, rot: Quat, pearls: &mut impl PearlProvider);

    fn set_local_scale(self, scale: Vec3, pearls: &mut impl PearlProvider);
    fn set_local_scale_no_sync(self, scale: Vec3, pearls: &mut impl PearlProvider);

    fn sync_children(self, pearls: &mut impl PearlProvider);

    fn clear_parent(self, pearls: &mut impl PearlProvider);
    fn set_parent(self, parent: Handle<Transform>, pearls: &mut impl PearlProvider);
}

impl TransformHandleExt for Handle<Transform> {
    fn set_local_pos(self, pos: Vec3, pearls: &mut impl PearlProvider) {
        self.set_local_pos_no_sync(pos, pearls);
        self.sync_children(pearls);
    }

    fn set_local_pos_no_sync(self, pos: Vec3, pearls: &mut impl PearlProvider) {
        let Some(transform) = pearls.get_mut(self) else { return };
        transform.local_pos = pos;
    }

    fn set_local_rot(self, rot: Quat, pearls: &mut impl PearlProvider) {
        self.set_local_rot_no_sync(rot, pearls);
        self.sync_children(pearls);
    }

    fn set_local_rot_no_sync(self, rot: Quat, pearls: &mut impl PearlProvider) {
        let Some(transform) = pearls.get_mut(self) else { return };
        transform.local_rot = rot;
    }

    fn set_local_scale(self, scale: Vec3, pearls: &mut impl PearlProvider) {
        self.set_local_scale_no_sync(scale, pearls);
        self.sync_children(pearls);
    }

    fn set_local_scale_no_sync(self, scale: Vec3, pearls: &mut impl PearlProvider) {
        let Some(transform) = pearls.get_mut(self) else { return };
        transform.local_scale = scale;
    }

    fn sync_children(self, pearls: &mut impl PearlProvider) {
        let Some(this_child) = pearls.get(self) else { return };
        let world_mat = this_child.calculate_world_mat();
        let children = this_child.children.clone();

        for child_handle in children.iter() {
            match pearls.get_mut(*child_handle) {
                Some(child) => {
                    child.parent_mat = world_mat;
                    Self::sync_children(*child_handle, pearls);
                }
                None => todo!(),
            }
        }
    }

    fn clear_parent(self, pearls: &mut impl PearlProvider) {
        // check if the transform_handle is valid
        if pearls.get_mut(self).is_none() {
            warn!("Tried to `clear_parent` with a handle that is invalid.");
            return;
        };

        // Set the parent
        force_replace_parent(self, None, pearls);
    }

    fn set_parent(self, parent_handle: Handle<Transform>, pearls: &mut impl PearlProvider) {
        if self == parent_handle {
            warn!("Tried to `set_parent` with identical handles.");
            return;
        }

        // check if the parent is valid
        if pearls.get(parent_handle).is_none() {
            warn!("Tried to `set_parent` with a parent handle that is invalid.");
            return;
        };

        // check if the transform_handle is valid
        let Some(child) = pearls.get_mut(self) else {
            warn!("Tried to `set_parent` with a child handle that is invalid.");
            return;
        };

        // check if parent is already correct, and if so, skip parenting process
        match child.parent {
            Some(current_parent) if current_parent == parent_handle => return,
            _ => (),
        };

        fn resolve_recursive(
            child_handle: Handle<Transform>,
            parent_handle: Handle<Transform>,
            current_handle: Handle<Transform>,
            pearls: &mut impl PearlProvider,
        ) {
            let current_transform = pearls.get_mut(current_handle).unwrap();
            match current_transform.parent {
                // if we find that the parent child relationship would be recursive, set the parent and resolve the recursion
                Some(next_parent) if next_parent == child_handle => {
                    // set the child parent, and get the old parent
                    let old_parent =
                        force_replace_parent(child_handle, Some(parent_handle), pearls);
                    // set the old parent as the new parents parent
                    force_replace_parent(parent_handle, old_parent, pearls);
                }
                // if we are not at the top of the chain yet, recurse again up the chain
                Some(next_parent) => {
                    resolve_recursive(child_handle, parent_handle, next_parent, pearls);
                }
                // if we got to the end of the chain, it is safe to just set the parent
                None => {
                    force_replace_parent(child_handle, Some(parent_handle), pearls);
                }
            }
        }

        // set parent and resolve recursive loops
        resolve_recursive(self, parent_handle, parent_handle, pearls);
    }
}

fn force_replace_parent(
    child_handle: Handle<Transform>,
    parent_handle_option: Option<Handle<Transform>>,
    pearls: &mut impl PearlProvider,
) -> Option<Handle<Transform>> {
    // replace the childs parent with new parent
    let child_transform = pearls.get_mut(child_handle).unwrap();
    let old_parent_option = std::mem::replace(&mut child_transform.parent, parent_handle_option);

    // remove the child from the old parents set, if there is one
    if let Some(old_parent_handle) = old_parent_option {
        if let Some(old_parent) = pearls.get_mut(old_parent_handle) {
            old_parent.children.remove(&child_handle);
        }
    }

    // add child to the new parents set, if there is one
    if let Some(parent_handle) = parent_handle_option {
        if let Some(parent) = pearls.get_mut(parent_handle) {
            parent.children.insert(child_handle);
        }
    }

    old_parent_option
}
