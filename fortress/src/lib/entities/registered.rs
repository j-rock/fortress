use crate::{
    entities::{
        Entity,
        EntityId,
    },
    physics::PhysicsSimulation,
};
use nphysics2d::object::{
    DefaultBodyHandle,
    DefaultColliderHandle
};

pub struct RegisteredCollider {
    physics_sim: PhysicsSimulation,
    handle: DefaultColliderHandle,
}

impl RegisteredCollider {
    pub fn new(handle: DefaultColliderHandle, entity: Entity, physics_sim: &PhysicsSimulation) -> RegisteredCollider {
        let physics_sim = physics_sim.clone();
        physics_sim.borrow_mut().register(EntityId::from_collider_handle(handle), entity);

        RegisteredCollider {
            physics_sim,
            handle,
        }
    }
}

impl Drop for RegisteredCollider {
    fn drop(&mut self) {
        self.physics_sim.borrow_mut().drop_collider(self.handle);
    }
}

pub struct RegisteredBody {
    pub physics_sim: PhysicsSimulation,
    pub handle: DefaultBodyHandle,
}

impl RegisteredBody {
    pub fn new(handle: DefaultBodyHandle, entity: Entity, physics_sim: &PhysicsSimulation) -> RegisteredBody {
        let physics_sim = physics_sim.clone();
        physics_sim.borrow_mut().register(EntityId::from_body_handle(handle), entity);

        RegisteredBody {
            physics_sim,
            handle,
        }
    }
}

impl Drop for RegisteredBody {
    fn drop(&mut self) {
        self.physics_sim.borrow_mut().drop_body(self.handle);
    }
}
