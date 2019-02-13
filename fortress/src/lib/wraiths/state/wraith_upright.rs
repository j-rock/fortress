use control::Controller;
use dimensions::time::DeltaTime;
use wraiths::{
    WraithState,
    state::WraithStateMachine,
};

#[derive(Default)]
pub struct WraithUpright;

impl WraithStateMachine for WraithUpright {
    fn pre_update(&mut self, _wraith_state: &mut WraithState, _controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn WraithStateMachine>> {
        None
    }
}

impl WraithUpright {
    pub fn new() -> WraithUpright {
        WraithUpright
    }
}
