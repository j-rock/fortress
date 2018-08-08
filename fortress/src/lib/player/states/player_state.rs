use control::Controller;
use dimensions::time::DeltaTime;

pub trait PlayerState {
    fn update(self, controller: &Controller, dt: DeltaTime) -> Box<dyn PlayerState>;
}