use app::StatusOr;
use control::{
    ControlEvent,
    GamepadControls,
    KeyboardControls
};
use dimensions::LrDirection;
use sdl2::{
    EventPump,
    keyboard::Scancode
};

pub struct Controller {
    keyboard: KeyboardControls,
    gamepad: GamepadControls,
}

impl Controller {
    pub fn new() -> StatusOr<Controller> {
        Ok(Controller {
            keyboard: KeyboardControls::new(),
            gamepad: GamepadControls::new()?,
        })
    }

    pub fn update(&mut self, e: &EventPump) {
        self.keyboard.update(e);
        self.gamepad.update();
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

    pub fn keyboard_used_first_time(&self) -> bool {
        self.keyboard.used_first_time()
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
