use std::f32::consts::PI;

use boba_core::PearlRegister;
use cgmath::{EuclideanSpace, InnerSpace, Point3, Quaternion, Vector3};

pub trait TransformDefault {
    fn default() -> Self;
}

impl TransformDefault for Quaternion<f32> {
    fn default() -> Self {
        Quaternion::from_sv(1., (0., 0., 0.).into())
    }
}

pub struct Transform {
    position: Point3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(
            (0., 0., 0.).into(),
            Quaternion::default(),
            (0., 0., 0.).into(),
        )
    }
}

impl PearlRegister for Transform {
    fn register(_: boba_core::Pearl<Self>, _: &mut boba_core::storage::StageRunners) {
        // do nothing for now
    }
}

impl Transform {
    pub fn from_position(position: Point3<f32>) -> Self {
        Self::new(position, Quaternion::default(), (0., 0., 0.).into())
    }

    pub fn new(position: Point3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn position(&self) -> &Point3<f32> {
        &self.position
    }

    pub fn rotation(&self) -> &Quaternion<f32> {
        &self.rotation
    }

    pub fn scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn set_position(&mut self, position: Point3<f32>) {
        self.position = position
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale
    }

    pub fn look_at(&mut self, point: Point3<f32>) {
        let forward = (point.to_vec() - self.position.to_vec()).normalize();
        let dot = cgmath::dot(Vector3::unit_z(), forward);
        if (dot + 1.).abs() < 0.000001 {
            self.rotation = Quaternion::new(PI, 0., 0., 1.);
            return;
        }
        if (dot - 1.).abs() < 0.000001 {
            self.rotation = Quaternion::new(1., 0., 0., 0.);
            return;
        }

        let angle = dot.acos();
        let axis = Vector3::unit_z().cross(forward).normalize();
        let half_angle = angle * 0.5;
        let s = half_angle.sin();
        self.rotation = Quaternion::new(half_angle.cos(), axis.x * s, axis.y * s, axis.z * s);
    }
}
