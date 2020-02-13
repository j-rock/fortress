use nphysics2d::object::{
    DefaultBodyHandle,
    DefaultColliderHandle
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EntityId {
    Collider(DefaultColliderHandle),
    RigidBody(DefaultBodyHandle),
}

impl EntityId {
    pub fn from_collider_handle(handle: DefaultColliderHandle) -> EntityId {
        EntityId::Collider(handle)
    }

    pub fn from_body_handle(handle: DefaultBodyHandle) -> EntityId {
        EntityId::RigidBody(handle)
    }
}
