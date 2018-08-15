use control::Controller;
use dimensions::time::DeltaTime;
use wraith::{
    WraithState,
    state::WraithStateMachine,
};

pub struct WraithUpright;

impl WraithStateMachine for WraithUpright {
    fn pre_update(&mut self, wraith_state: &mut WraithState, _controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn WraithStateMachine>> {
        wraith_state.body.move_horizontal(0.0, None);
        None
    }
}

impl WraithUpright {
    pub fn new() -> WraithUpright {
        WraithUpright
    }
}
