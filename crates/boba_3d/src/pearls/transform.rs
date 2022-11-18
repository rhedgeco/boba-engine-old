use boba_core::{storage::StageRunners, Pearl, PearlId, PearlRegister};
use glam::{Mat4, Quat, Vec3, Vec4};
use indexmap::IndexMap;
use log::error;

pub struct BobaTransform {
    world_position: Vec3,
    world_rotation: Quat,
    lossy_scale: Vec3,

    local_position: Vec3,
    local_rotation: Quat,
    local_scale: Vec3,

    parent_matrix: Mat4,
    local_matrix: Mat4,

    parent: Option<Pearl<BobaTransform>>,
    children: IndexMap<PearlId, Pearl<BobaTransform>>,
}

impl Default for BobaTransform {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }
}

impl PearlRegister for BobaTransform {
    fn register(_: Pearl<Self>, _: &mut StageRunners) {
        // do nothing for now
    }
}

impl BobaTransform {
    pub fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let matrix = Mat4::from_scale_rotation_translation(scale, rotation, position);

        Self {
            world_position: position,
            world_rotation: rotation,
            lossy_scale: scale,

            local_position: position,
            local_rotation: rotation,
            local_scale: scale,

            parent_matrix: Mat4::IDENTITY,
            local_matrix: matrix,

            parent: None,
            children: IndexMap::default(),
        }
    }

    pub fn world_position(&self) -> Vec3 {
        self.world_position
    }

    pub fn world_rotation(&self) -> Quat {
        self.world_rotation
    }

    pub fn lossy_scale(&self) -> Vec3 {
        self.lossy_scale
    }

    pub fn local_position(&self) -> Vec3 {
        self.local_position
    }

    pub fn local_rotation(&self) -> Quat {
        self.local_rotation
    }

    pub fn local_scale(&self) -> Vec3 {
        self.local_scale
    }

    pub fn world_matrix(&self) -> Mat4 {
        self.parent_matrix * self.local_matrix
    }

    pub fn local_matrix(&self) -> Mat4 {
        self.local_matrix
    }

    pub fn add_child(&mut self, child: Pearl<BobaTransform>) {
        let Ok(child_data) = child.data_mut() else {
            error!("Failed to set child transform because it is currently borrowed.");
            return;
        };

        if child_data.parent.is_none() {
            drop(child_data);
            self.children.insert(*child.id(), child);
            return;
        }

        let Ok(mut parent_data) = child_data.parent.as_ref().unwrap().data_mut() else {
            error!("Failed to set child transform because its parent is currently borrowed.");
            return;
        };

        parent_data.children.remove(child.id());
        drop(parent_data);
        drop(child_data);
        self.children.insert(*child.id(), child);
    }

    /// Sets the local position of the transform.
    ///
    /// Also recalculates the world position, and distributes the changes to
    /// all available children.
    pub fn set_local_position(&mut self, position: Vec3) {
        self.local_position = position;
        self.local_matrix.w_axis = Vec4::from((self.local_position, 1.0));
        self.calculate_world_transforms();
        self.apply_matrix_to_children();
    }

    /// Sets the local rotation of the transform.
    ///
    /// Also recalculates the world rotation, and distributes the changes to
    /// all available children.
    pub fn set_local_rotation(&mut self, rotation: Quat) {
        self.local_rotation = rotation;
        self.local_matrix = Mat4::from_scale_rotation_translation(
            self.local_scale,
            self.local_rotation,
            self.local_position,
        );
        self.calculate_world_transforms();
        self.apply_matrix_to_children();
    }

    pub fn look_at(&mut self, point: Vec3) {
        let Some(look_vector) = (point - self.local_position).try_normalize() else {
            return;
        };

        let dot = Vec3::Z.dot(look_vector);
        if (dot + 1.).abs() < 0.000001 {
            self.local_rotation = Quat::from_axis_angle(Vec3::Y, 0.);
            return;
        }
        if (dot - 1.).abs() < 0.000001 {
            self.local_rotation = Quat::from_axis_angle(Vec3::Y, 180.);
            return;
        }

        let angle = dot.acos();
        let axis = Vec3::Z.cross(look_vector).normalize();
        self.local_rotation = Quat::from_axis_angle(axis, angle);
    }

    fn calculate_world_transforms(&mut self) {
        (self.world_position, self.world_rotation, self.lossy_scale) =
            self.world_matrix().to_scale_rotation_translation();
    }

    fn apply_matrix_to_children(&mut self) {
        for child in self.children.values() {
            match child.data_mut() {
                Err(e) => {
                    error!("Could not sync child transform due to: {e}");
                    continue;
                }
                Ok(mut transform) => {
                    transform.parent_matrix = self.world_matrix();
                    transform.calculate_world_transforms();
                    transform.apply_matrix_to_children();
                }
            }
        }
    }
}
