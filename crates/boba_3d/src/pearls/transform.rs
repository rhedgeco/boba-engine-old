use boba_core::PearlRegister;
use glam::{Quat, Vec3};

pub struct BobaTransform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Default for BobaTransform {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }
}

impl PearlRegister for BobaTransform {
    fn register(_: boba_core::Pearl<Self>, _: &mut boba_core::storage::StageRunners) {
        // do nothing for now
    }
}

impl BobaTransform {
    pub fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale
    }

    pub fn look_at(&mut self, point: Vec3) {
        let Some(look_vector) = (point - self.position).try_normalize() else {
            return;
        };

        let dot = Vec3::Z.dot(look_vector);
        if (dot + 1.).abs() < 0.000001 {
            self.rotation = Quat::from_axis_angle(Vec3::Y, 0.);
            return;
        }
        if (dot - 1.).abs() < 0.000001 {
            self.rotation = Quat::from_axis_angle(Vec3::Y, 180.);
            return;
        }

        let angle = dot.acos();
        let axis = Vec3::Z.cross(look_vector).normalize();
        self.rotation = Quat::from_axis_angle(axis, angle);
    }
}
