use control::Controller;
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use wraith::{
    WraithState,
    state::WraithStateMachine,
};

pub struct WraithUpright;

impl WraithStateMachine for WraithUpright {
    fn pre_update(&mut self, _wraith_state: &mut WraithState, _controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn WraithStateMachine>> {
        None
    }

    fn take_slashing(&mut self, wraith_state: &mut WraithState, dir: LrDirection) {
        wraith_state.body.move_horizontal(wraith_state.config.slashed_speed, Some(dir));
    }
}

impl WraithUpright {
    pub fn new() -> WraithUpright {
        WraithUpright
    }
}
