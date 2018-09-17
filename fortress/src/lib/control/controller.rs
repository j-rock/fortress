use control::{
    ControlEvent,
    ControllerEvent,
    GamepadControls,
    KeyboardControls
};
use dimensions::LrDirection;
use sdl2::{
    EventPump,
    keyboard::Scancode,
    self,
};

pub struct Controller {
    keyboard: KeyboardControls,
    gamepad: GamepadControls,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            keyboard: KeyboardControls::new(),
            gamepad: GamepadControls::new(),
        }
    }

    pub fn ingest_gamepad_events(&mut self, controller_subsystem: &sdl2::GameControllerSubsystem, gamepad_events: Vec<sdl2::event::Event>) {
        self.gamepad.ingest_gamepad_events(controller_subsystem, gamepad_events);
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

    pub fn controller_events(&self) -> Vec<ControllerEvent> {
        let mut controller_events = self.gamepad.controller_events().clone();
        if self.keyboard.just_joined() {
            controller_events.push(ControllerEvent::KeyboardUsed);
        }

        controller_events
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
