use boba_core::{Pearl, PearlId, PearlMutError};
use glam::{Mat4, Quat, Vec3, Vec4};
use indexmap::IndexSet;
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
    children: IndexSet<Pearl<BobaTransform>>,
}

impl Default for BobaTransform {
    fn default() -> Self {
        Self::from_position(Vec3::ZERO)
    }
}

impl BobaTransform {
    pub fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn from_position_look_at(position: Vec3, look: Vec3) -> Self {
        let mut new = Self::new(position, Quat::IDENTITY, Vec3::ONE);
        new.look_at(look);
        new
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
            children: Default::default(),
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
        self.local_matrix * self.parent_matrix
    }

    pub fn local_matrix(&self) -> Mat4 {
        self.local_matrix
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
        self.set_local_rotation(Quat::from_axis_angle(axis, angle));
    }

    fn calculate_world_transforms(&mut self) {
        (self.lossy_scale, self.world_rotation, self.world_position) =
            self.world_matrix().to_scale_rotation_translation();
    }

    fn apply_matrix_to_children(&mut self) {
        for child in self.children.iter() {
            match child.borrow_mut() {
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

pub enum SetParentError {
    RecursionError,
    PearlError(PearlMutError),
}

pub trait SetTransformParent {
    fn set_parent(&mut self, parent: Pearl<BobaTransform>) -> Result<(), SetParentError>;
}

impl SetTransformParent for Pearl<BobaTransform> {
    fn set_parent(&mut self, parent: Pearl<BobaTransform>) -> Result<(), SetParentError> {
        if self.id() == parent.id() {
            return Err(SetParentError::RecursionError);
        }

        let mut self_data = match self.borrow_mut() {
            Ok(d) => d,
            Err(e) => return Err(SetParentError::PearlError(e)),
        };

        let mut parent_data = match parent.borrow_mut() {
            Ok(d) => d,
            Err(e) => return Err(SetParentError::PearlError(e)),
        };

        validate_parent_recursive(self.id(), &*parent_data)?;

        let Some(self_parent) = &self_data.parent else {
            parent_data.children.insert(self.clone());
            self_data.parent = Some(parent.clone());
            return Ok(());
        };

        let mut self_parent_data = match self_parent.borrow_mut() {
            Ok(d) => d,
            Err(e) => return Err(SetParentError::PearlError(e)),
        };

        self_parent_data.children.remove(self);

        drop(self_parent_data);
        parent_data.children.insert(self.clone());
        self_data.parent = Some(parent.clone());

        Ok(())
    }
}

fn validate_parent_recursive(id: &PearlId, target: &BobaTransform) -> Result<(), SetParentError> {
    let Some(parent) = &target.parent else {
        return Ok(());
    };

    if id == parent.id() {
        return Err(SetParentError::RecursionError);
    };

    let parent_data = match parent.borrow_mut() {
        Ok(d) => d,
        Err(e) => return Err(SetParentError::PearlError(e)),
    };

    return validate_parent_recursive(id, &*parent_data);
}
