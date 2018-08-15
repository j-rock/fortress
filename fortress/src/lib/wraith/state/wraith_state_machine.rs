use control::Controller;
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use wraith::WraithState;

pub trait WraithStateMachine {
    // Before physics step.
    fn pre_update(&mut self, wraith_state: &mut WraithState, controller: &Controller, dt: DeltaTime) -> Option<Box<dyn WraithStateMachine>>;

    // After physics step.
    fn post_update(&mut self) -> Option<Box<dyn WraithStateMachine>> {
        None
    }

    fn take_slashing(&mut self, _wraith_state: &mut WraithState, _dir: LrDirection) {}
}
