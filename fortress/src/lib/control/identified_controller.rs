use crate::control::{
    ControlEvent,
    Controller,
    ControllerId,
};

#[derive(Copy, Clone)]
pub struct IdentifiedController<'a> {
    controller: &'a Controller,
    controller_id: ControllerId
}

impl<'a> IdentifiedController<'a> {
    pub fn new(controller: &'a Controller, controller_id: ControllerId) -> Self {
        IdentifiedController {
            controller,
            controller_id
        }
    }

    pub fn is_pressed(&self, event: ControlEvent) -> bool {
        self.controller.is_pressed(self.controller_id, event)
    }

    pub fn just_pressed(&self, event: ControlEvent) -> bool {
        self.controller.just_pressed(self.controller_id, event)
    }

    pub fn just_released(&self, event: ControlEvent) -> bool {
        self.controller.just_released(self.controller_id, event)
    }
}

