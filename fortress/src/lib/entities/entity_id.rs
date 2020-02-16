use nphysics2d::object::DefaultBodyHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EntityId(DefaultBodyHandle);

impl EntityId {
    pub fn from_body_handle(handle: DefaultBodyHandle) -> EntityId {
        EntityId(handle)
    }
}
