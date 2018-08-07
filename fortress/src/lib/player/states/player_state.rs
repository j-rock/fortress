use control::Controller;
use dimensions::time::DeltaTime;
use entity::EntityRegistrar;

pub trait PlayerState {
    fn update(self, controller: &Controller, registrar: &mut EntityRegistrar, dt: DeltaTime) -> Box<dyn PlayerState>;
}