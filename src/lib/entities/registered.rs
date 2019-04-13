use crate::{
    entities::{
        Entity,
        EntityId,
    },
    physics::PhysicsSimulation,
};
use nphysics2d::object::{
    BodyHandle,
    ColliderHandle
};

pub struct RegisteredCollider {
    physics_sim: PhysicsSimulation,
    handle: ColliderHandle,
}

impl RegisteredCollider {
    pub fn new(handle: ColliderHandle, entity: Entity, physics_sim: &PhysicsSimulation) -> RegisteredCollider {
        let physics_sim = physics_sim.clone();
        physics_sim.borrow_mut().registrar_mut().register(EntityId::from_collider_handle(handle), entity);

        RegisteredCollider {
            physics_sim,
            handle,
        }
    }
}

impl Drop for RegisteredCollider {
    fn drop(&mut self) {
        self.physics_sim.borrow_mut().registrar_mut().unregister(EntityId::from_collider_handle(self.handle));
        self.physics_sim.borrow_mut().world_mut().remove_colliders(&[self.handle]);
    }
}

pub struct RegisteredBody {
    pub physics_sim: PhysicsSimulation,
    pub handle: BodyHandle,
}

impl RegisteredBody {
    pub fn new(handle: BodyHandle, entity: Entity, physics_sim: &PhysicsSimulation) -> RegisteredBody {
        let physics_sim = physics_sim.clone();
        physics_sim.borrow_mut().registrar_mut().register(EntityId::from_body_handle(handle), entity);

        RegisteredBody {
            physics_sim,
            handle,
        }
    }
}

impl Drop for RegisteredBody {
    fn drop(&mut self) {
        self.physics_sim.borrow_mut().registrar_mut().unregister(EntityId::from_body_handle(self.handle));
        self.physics_sim.borrow_mut().world_mut().remove_bodies(&[self.handle]);
    }
}
