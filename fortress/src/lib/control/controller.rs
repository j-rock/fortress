use control::{
    ControlEvent,
    KeyboardControls
};
use dimensions::LrDirection;
use sdl2::{
    EventPump,
    keyboard::Scancode
};

pub struct Controller {
    keyboard: KeyboardControls,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            keyboard: KeyboardControls::new()
        }
    }

    pub fn update(&mut self, e: &EventPump) {
        self.keyboard.update(e);
    }

    pub fn is_pressed(&self, event: ControlEvent) -> bool {
        self.keyboard.is_pressed(self.control_event_to_scancode(event))
    }

    pub fn just_pressed(&self, event: ControlEvent) -> bool {
        self.keyboard.just_pressed(self.control_event_to_scancode(event))
    }

    pub fn just_released(&self, event: ControlEvent) -> bool {
        self.keyboard.just_released(self.control_event_to_scancode(event))
    }

    fn control_event_to_scancode(&self, event: ControlEvent) -> Scancode {
        match event {
            ControlEvent::PlayerFire => Scancode::I,
            ControlEvent::PlayerJump => Scancode::Space,
            ControlEvent::PlayerMove(LrDirection::Left) => Scancode::A,
            ControlEvent::PlayerMove(LrDirection::Right) => Scancode::D,
            ControlEvent::PlayerSlash => Scancode::J,
            ControlEvent::RespawnEntities => Scancode::R,
        }
    }
}
