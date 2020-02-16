use crate::{
    entities::{
        Entity,
        EntityId,
    },
    physics::PhysicsSimulation,
};
use nalgebra::{
    Isometry2,
    Point2,
    Translation2,
    UnitComplex,
    Vector2,
};
use nphysics2d::{
    algebra::{
        Force2,
        ForceType
    },
    object::{
        Body,
        BodyPartHandle,
        ColliderDesc,
        DefaultBodyHandle,
        RigidBody,
    }
};

pub struct RegisteredBodyBuilder<BodyT, EntityT> {
    rigid_body: BodyT,
    colliders: Vec<ColliderDesc<f64>>,
    entity: EntityT,
}

impl RegisteredBodyBuilder<(), ()> {
    pub fn new() -> Self {
        RegisteredBodyBuilder {
            rigid_body: (),
            colliders: Vec::with_capacity(1),
            entity: ()
        }
    }
}

impl<BodyT, EntityT> RegisteredBodyBuilder<BodyT, EntityT> {
    pub fn rigid_body(self, rigid_body: RigidBody<f64>) -> RegisteredBodyBuilder<RigidBody<f64>, EntityT> {
        RegisteredBodyBuilder {
            rigid_body,
            colliders: self.colliders,
            entity: self.entity
        }
    }

    pub fn collider(mut self, collider_desc: ColliderDesc<f64>) -> Self {
        self.colliders.push(collider_desc);
        self
    }

    pub fn add_collider(&mut self, collider_desc: ColliderDesc<f64>) {
        self.colliders.push(collider_desc);
    }

    pub fn entity(self, entity: Entity) -> RegisteredBodyBuilder<BodyT, Entity> {
        RegisteredBodyBuilder {
            rigid_body: self.rigid_body,
            colliders: self.colliders,
            entity,
        }
    }
}

impl RegisteredBodyBuilder<RigidBody<f64>, Entity> {
    pub fn build(self, physics_sim: &mut PhysicsSimulation) -> RegisteredBody {
        let rigid_body_handle = physics_sim.borrow_mut().add_rigid_body(self.rigid_body);
        for collider_desc in self.colliders.into_iter() {
            let collider = collider_desc.build(BodyPartHandle(rigid_body_handle, 0));
            physics_sim.borrow_mut().add_collider(collider);
        }
        RegisteredBody::new(rigid_body_handle, self.entity, physics_sim)
    }
}

pub struct RegisteredBody {
    physics_sim: PhysicsSimulation,
    handle: DefaultBodyHandle,
}

impl RegisteredBody {
    fn new(handle: DefaultBodyHandle, entity: Entity, physics_sim: &PhysicsSimulation) -> RegisteredBody {
        let physics_sim = physics_sim.clone();
        physics_sim.borrow_mut().register(EntityId::from_body_handle(handle), entity);

        RegisteredBody {
            physics_sim,
            handle,
        }
    }

    pub fn default_position(&self) -> Option<Point2<f64>> {
        let physics_sim = self.physics_sim.borrow();
        let body = physics_sim.get_rigid_body(self.handle)?;
        Some(Point2::from(body.position().translation.vector))
    }

    pub fn default_set_position(&mut self, point: Point2<f64>) {
        let mut physics_sim = self.physics_sim.borrow_mut();
        if let Some(body) = physics_sim.get_rigid_body_mut(self.handle) {
            let translation = Translation2::from(point.coords);
            body.set_position(Isometry2::from_parts(translation, UnitComplex::identity()));
        }
    }

    pub fn default_velocity(&self) -> Option<Vector2<f64>> {
        let physics_sim = self.physics_sim.borrow();
        let body = physics_sim.get_rigid_body(self.handle)?;
        Some(body.velocity().linear)
    }

    pub fn default_set_velocity(&mut self, desired_velocity: Vector2<f64>) {
        let mut physics_sim = self.physics_sim.borrow_mut();
        if let Some(body) = physics_sim.get_rigid_body_mut(self.handle) {
            let actual_body_velocity = body.velocity().linear;
            let mass = body.augmented_mass().linear;
            let impulse = Force2::linear(mass * (desired_velocity - actual_body_velocity));
            body.apply_force(0, &impulse, ForceType::Impulse, true /*one-time impulse*/);
        }
    }
}

impl Drop for RegisteredBody {
    fn drop(&mut self) {
        self.physics_sim.borrow_mut().drop_body(self.handle);
    }
}
