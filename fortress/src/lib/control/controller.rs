use app::StatusOr;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use control::{
    ControlEvent,
    ControllerId,
    ControllerEvent,
    GamepadControls,
    KeyboardControls
};
use sdl2::{
    EventPump,
    self,
};

pub struct Controller {
    keyboard: KeyboardControls,
    gamepad: GamepadControls,
}

impl Controller {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Controller> {
        let config_manager = SimpleConfigManager::new(config_watcher, "gamepad.conf")?;
        Ok(Controller {
            keyboard: KeyboardControls::new(),
            gamepad: GamepadControls::new(config_manager),
        })
    }

    pub fn ingest_gamepad_events(&mut self, controller_subsystem: &sdl2::GameControllerSubsystem, gamepad_events: Vec<sdl2::event::Event>) {
        self.gamepad.ingest_gamepad_events(controller_subsystem, gamepad_events);
    }

    pub fn update(&mut self, e: &EventPump) {
        self.keyboard.update(e);
    }

    pub fn is_pressed(&self, controller_id: ControllerId, event: ControlEvent) -> bool {
        match controller_id {
            ControllerId::Keyboard => {
                self.keyboard.is_pressed(event)
            },
            ControllerId::Gamepad(gamepad_id) => {
                self.gamepad.is_pressed(gamepad_id, event)
            }
        }
    }

    pub fn just_pressed(&self, controller_id: ControllerId, event: ControlEvent) -> bool {
        match controller_id {
            ControllerId::Keyboard => {
                self.keyboard.just_pressed(event)
            },
            ControllerId::Gamepad(gamepad_id) => {
                self.gamepad.just_pressed(gamepad_id, event)
            }
        }
    }

    pub fn just_released(&self, controller_id: ControllerId, event: ControlEvent) -> bool {
        match controller_id {
            ControllerId::Keyboard => {
                self.keyboard.just_released(event)
            },
            ControllerId::Gamepad(gamepad_id) => {
                self.gamepad.just_released(gamepad_id, event)
            }
        }
    }

    pub fn controller_events(&self) -> Vec<ControllerEvent> {
        let mut controller_events = self.gamepad.controller_events().clone();
        if self.keyboard.just_joined() {
            controller_events.push(ControllerEvent::KeyboardUsed);
        }

        controller_events
    }
}
