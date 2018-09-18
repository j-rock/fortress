use control::{
    Controller,
    ControllerId,
};
use dimensions::time::DeltaTime;
use player::PlayerState;

pub trait PlayerStateMachine {
    // Before physics step.
    fn pre_update(&mut self, player_state: &mut PlayerState, controller_id: ControllerId, controller: &Controller, dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>>;

    // After physics step.
    fn post_update(&mut self) -> Option<Box<dyn PlayerStateMachine>> {
        None
    }

    fn make_foot_contact(&mut self) {}
}