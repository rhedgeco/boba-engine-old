use boba_3d::{glam::Vec3, pearls::BobaTransform};
use boba_core::Pearl;
use log::error;
use rapier3d::prelude::{
    BroadPhase, CCDSolver, Collider, ColliderSet, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBody, RigidBodyHandle,
    RigidBodySet,
};

struct RigidBodyConnection {
    handle: RigidBodyHandle,
    transform: Pearl<BobaTransform>,
}

impl RigidBodyConnection {
    fn sync(&mut self, rigid_body_set: &RigidBodySet) {
        let mut transform = match self.transform.borrow_mut() {
            Ok(t) => t,
            Err(e) => {
                error!("Error syncing physics transform. Error: {e}");
                return;
            }
        };

        let sync_data = &rigid_body_set[self.handle];
        transform.set_local_position(sync_data.position().translation.into());
        transform.set_local_rotation(sync_data.position().rotation.into());
    }
}

pub struct RapierPhysics {
    pub gravity: Vec3,

    connections: Vec<RigidBodyConnection>,

    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl Default for RapierPhysics {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., -9.81, 0.),
            connections: Vec::new(),
            integration_parameters: Default::default(),
            physics_pipeline: Default::default(),
            island_manager: Default::default(),
            broad_phase: Default::default(),
            narrow_phase: Default::default(),
            rigid_body_set: Default::default(),
            collider_set: Default::default(),
            impulse_joint_set: Default::default(),
            multibody_joint_set: Default::default(),
            ccd_solver: Default::default(),
            physics_hooks: Default::default(),
            event_handler: Default::default(),
        }
    }
}

impl RapierPhysics {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity.into(),
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );

        for connection in &mut self.connections {
            connection.sync(&self.rigid_body_set);
        }
    }

    pub fn create_transform(
        &mut self,
        rigidbody: RigidBody,
        collider: Collider,
    ) -> Pearl<BobaTransform> {
        let transform = Pearl::wrap(BobaTransform::from_position_rotation(
            rigidbody.position().translation.into(),
            rigidbody.position().rotation.into(),
        ));

        let handle = self.rigid_body_set.insert(rigidbody);
        self.collider_set
            .insert_with_parent(collider, handle, &mut self.rigid_body_set);

        let connection = RigidBodyConnection {
            handle,
            transform: transform.clone(),
        };

        self.connections.push(connection);

        transform
    }

    pub fn create_transform_multi_collider(
        &mut self,
        rigidbody: RigidBody,
        colliders: Vec<Collider>,
    ) -> Pearl<BobaTransform> {
        let transform = Pearl::wrap(BobaTransform::from_position_rotation(
            rigidbody.position().translation.into(),
            rigidbody.position().rotation.into(),
        ));

        let handle = self.rigid_body_set.insert(rigidbody);
        for collider in &colliders {
            self.collider_set.insert_with_parent(
                collider.clone(),
                handle,
                &mut self.rigid_body_set,
            );
        }

        let connection = RigidBodyConnection {
            handle,
            transform: transform.clone(),
        };

        self.connections.push(connection);

        transform
    }
}
