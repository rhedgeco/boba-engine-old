use boba_core::{
    macros::Pearl,
    pearls::{
        map::{EventPearls, Handle},
        Pearl,
    },
};
use glam::{Mat4, Quat, Vec3, Vec4};
use indexmap::IndexSet;

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

    pub fn set_local_pos(&mut self, pos: Vec3, pearls: &mut EventPearls) {
        self.set_local_pos_no_sync(pos);
        self.sync_children(pearls);
    }

    pub fn set_local_rot(&mut self, rot: Quat, pearls: &mut EventPearls) {
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

    pub fn sync_children(&mut self, pearls: &mut EventPearls) {
        self.sync_world_transforms();
        let world_mat = self.world_mat();

        for child_handle in self.children.iter() {
            match pearls.get_mut(*child_handle) {
                Some(child) => {
                    child.parent_mat = world_mat;
                    child.sync_world_transforms();
                    Self::sync_children_nested(*child_handle, pearls);
                }
                None => todo!(),
            }
        }
    }

    fn sync_children_nested(handle: Handle<Self>, pearls: &mut EventPearls) {
        let Some(this_child) = pearls.get(handle) else { return };
        let world_mat = this_child.world_mat();
        let children = this_child.children.clone();

        for child_handle in children.iter() {
            match pearls.get_mut(*child_handle) {
                Some(child) => {
                    child.parent_mat = world_mat;
                    child.sync_world_transforms();
                    Self::sync_children_nested(*child_handle, pearls);
                }
                None => todo!(),
            }
        }
    }
}
