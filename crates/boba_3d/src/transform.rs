use boba_core::{
    macros::Pearl,
    pearls::{
        map::{Handle, PearlProvider},
        Pearl,
    },
};
use glam::{Mat4, Quat, Vec3, Vec4};
use indexmap::IndexSet;
use log::warn;

#[derive(Pearl)]
pub struct Transform {
    world_pos: Vec3,
    world_rot: Quat,
    lossy_scale: Vec3,

    local_pos: Vec3,
    local_rot: Quat,
    local_scale: Vec3,

    parent_mat: Mat4,
    local_mat: Mat4,

    parent: Option<Handle<Self>>,
    children: IndexSet<Handle<Self>>,
}

impl Transform {
    pub fn new(pos: Vec3, rot: Quat, scale: Vec3) -> Self {
        let matrix = Mat4::from_scale_rotation_translation(scale, rot, pos);

        Self {
            world_pos: pos,
            world_rot: rot,
            lossy_scale: scale,

            local_pos: pos,
            local_rot: rot,
            local_scale: scale,

            parent_mat: Mat4::IDENTITY,
            local_mat: matrix,

            parent: None,
            children: Default::default(),
        }
    }

    pub fn from_pos(pos: Vec3) -> Self {
        Self::new(pos, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn from_rot(rot: Quat) -> Self {
        Self::new(Vec3::ZERO, rot, Vec3::ONE)
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    pub fn from_pos_rot(pos: Vec3, rot: Quat) -> Self {
        Self::new(pos, rot, Vec3::ONE)
    }

    pub fn from_pos_scale(pos: Vec3, scale: Vec3) -> Self {
        Self::new(pos, Quat::IDENTITY, scale)
    }

    pub fn from_rot_scale(rot: Quat, scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, rot, scale)
    }

    pub fn world_pos(&self) -> Vec3 {
        self.world_pos
    }

    pub fn world_rot(&self) -> Quat {
        self.world_rot
    }

    pub fn lossy_scale(&self) -> Vec3 {
        self.lossy_scale
    }

    pub fn local_pos(&self) -> Vec3 {
        self.local_pos
    }

    pub fn local_rot(&self) -> Quat {
        self.local_rot
    }

    pub fn local_scale(&self) -> Vec3 {
        self.local_scale
    }

    pub fn world_mat(&self) -> Mat4 {
        self.parent_mat * self.local_mat
    }

    pub fn local_mat(&self) -> Mat4 {
        self.local_mat
    }

    pub fn set_local_pos_sync(&mut self, pos: Vec3, pearls: &mut impl PearlProvider) {
        self.set_local_pos_no_sync(pos);
        self.sync_children(pearls);
    }

    pub fn set_local_rot_sync(&mut self, rot: Quat, pearls: &mut impl PearlProvider) {
        self.set_local_rot_no_sync(rot);
        self.sync_children(pearls);
    }

    pub fn set_local_pos_no_sync(&mut self, pos: Vec3) {
        self.local_pos = pos;
        self.local_mat.w_axis = Vec4::from((self.local_pos, 1.0));
    }

    pub fn set_local_rot_no_sync(&mut self, rot: Quat) {
        self.local_rot = rot;
        self.local_mat =
            Mat4::from_scale_rotation_translation(self.local_scale, self.local_rot, self.local_pos);
    }

    pub fn sync_world_transforms(&mut self) {
        (self.lossy_scale, self.world_rot, self.world_pos) =
            self.world_mat().to_scale_rotation_translation()
    }

    pub fn sync_children(&mut self, pearls: &mut impl PearlProvider) {
        self.sync_world_transforms();
        let world_mat = self.world_mat();

        for child_handle in self.children.iter() {
            match pearls.get_mut(*child_handle) {
                Some(child) => {
                    child.parent_mat = world_mat;
                    child.sync_world_transforms();
                    Self::sync_children_recursive(*child_handle, pearls);
                }
                None => todo!(),
            }
        }
    }

    fn sync_children_recursive(handle: Handle<Self>, pearls: &mut impl PearlProvider) {
        let Some(this_child) = pearls.get(handle) else { return };
        let world_mat = this_child.world_mat();
        let children = this_child.children.clone();

        for child_handle in children.iter() {
            match pearls.get_mut(*child_handle) {
                Some(child) => {
                    child.parent_mat = world_mat;
                    child.sync_world_transforms();
                    Self::sync_children_recursive(*child_handle, pearls);
                }
                None => todo!(),
            }
        }
    }

    pub fn clear_parent(handle: Handle<Self>, pearls: &mut impl PearlProvider) {
        // check if the transform_handle is valid
        if pearls.get_mut(handle).is_none() {
            warn!("Tried to `clear_parent` with a handle that is invalid.");
            return;
        };

        // Set the parent
        Self::force_replace_parent(handle, None, pearls);
    }

    pub fn set_parent(
        child_handle: Handle<Self>,
        parent_handle: Handle<Self>,
        pearls: &mut impl PearlProvider,
    ) {
        if child_handle == parent_handle {
            warn!("Tried to `set_parent` with identical handles.");
            return;
        }

        // check if the parent is valid
        if pearls.get(parent_handle).is_none() {
            warn!("Tried to `set_parent` with a parent handle that is invalid.");
            return;
        };

        // check if the transform_handle is valid
        let Some(child) = pearls.get_mut(child_handle) else {
            warn!("Tried to `set_parent` with a child handle that is invalid.");
            return;
        };

        // check if parent is already correct, and if so, skip parenting process
        match child.parent {
            Some(current_parent) if current_parent == parent_handle => return,
            _ => (),
        };

        // set parent and resolve recursive loops
        Self::parent_resolve_recursive(child_handle, parent_handle, parent_handle, pearls);
    }

    fn parent_resolve_recursive(
        child_handle: Handle<Self>,
        parent_handle: Handle<Self>,
        current_handle: Handle<Self>,
        pearls: &mut impl PearlProvider,
    ) {
        let current_transform = pearls.get_mut(current_handle).unwrap();
        match current_transform.parent {
            // if we find that the parent child relationship would be recursive, set the parent and resolve the recursion
            Some(next_parent) if next_parent == child_handle => {
                // set the child parent, and get the old parent
                let old_parent =
                    Self::force_replace_parent(child_handle, Some(parent_handle), pearls);
                // set the old parent as the new parents parent
                Self::force_replace_parent(parent_handle, old_parent, pearls);
            }
            // if we are not at the top of the chain yet, recurse again up the chain
            Some(next_parent) => {
                Self::parent_resolve_recursive(child_handle, parent_handle, next_parent, pearls)
            }
            // if we got to the end of the chain, it is safe to just set the parent
            None => {
                Self::force_replace_parent(child_handle, Some(parent_handle), pearls);
            }
        }
    }

    /// Forcefully sets a parent and returns the old one
    ///
    /// # Panics
    /// Will panic if the child handle is invalid
    fn force_replace_parent(
        child_handle: Handle<Self>,
        parent_handle_option: Option<Handle<Self>>,
        pearls: &mut impl PearlProvider,
    ) -> Option<Handle<Self>> {
        // replace the childs parent with new parent
        let child_transform = pearls.get_mut(child_handle).unwrap();
        let old_parent_option =
            std::mem::replace(&mut child_transform.parent, parent_handle_option);

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
}
