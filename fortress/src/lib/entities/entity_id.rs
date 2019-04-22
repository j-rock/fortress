use nphysics2d::object::{
    BodyHandle,
    ColliderHandle
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EntityId {
    Collider(ColliderHandle),
    RigidBody(BodyHandle),
}

impl EntityId {
    pub fn from_collider_handle(handle: ColliderHandle) -> EntityId {
        EntityId::Collider(handle)
    }

    pub fn from_body_handle(handle: BodyHandle) -> EntityId {
        EntityId::RigidBody(handle)
    }
}
