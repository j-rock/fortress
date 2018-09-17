use control::{
    ControllerEvent,
    GamepadId
};
use sdl2::{
    controller::GameController,
    event::Event,
    self
};
use std::collections::{
    HashMap,
    HashSet
};

pub struct GamepadControls {
    controller_events: Vec<ControllerEvent>,
    gamepads: HashMap<GamepadId, GameController>,

    currently_pressed: HashSet<ButtonState>,
    just_pressed: HashSet<ButtonState>,
    just_released: HashSet<ButtonState>,
}

impl GamepadControls {
    pub fn new() -> GamepadControls {
        GamepadControls {
            controller_events: Vec::new(),
            gamepads: HashMap::new(),
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_pressed(&self, button_state: ButtonState) -> bool {
        self.currently_pressed.contains(&button_state)
    }

    pub fn just_pressed(&self, button_state: ButtonState) -> bool {
        self.just_pressed.contains(&button_state)
    }

    pub fn just_released(&self, button_state: ButtonState) -> bool {
        self.just_released.contains(&button_state)
    }

    pub fn controller_events(&self) -> &Vec<ControllerEvent> {
        &self.controller_events
    }

    pub fn ingest_gamepad_events(&mut self, controller_subsystem: &sdl2::GameControllerSubsystem, gamepad_events: Vec<Event>) {
        self.controller_events.clear();
        self.just_released.clear();
        self.just_pressed.clear();

        for event in gamepad_events.into_iter() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Some(game_controller) = controller_subsystem.open(which).ok() {
                        let gamepad_id = GamepadId::from_u32(which);
                        self.gamepads.insert(gamepad_id, game_controller);
                        self.controller_events.push(ControllerEvent::GamepadConnected(gamepad_id));
                    } else {
                        println!("Couldn't open gamepad {}", which);
                    }
                },
                Event::ControllerDeviceRemoved { which, .. } => {
                    let gamepad_id = GamepadId::from_u32(which as u32);
                    self.gamepads.remove(&gamepad_id);
                },
                Event::ControllerButtonDown { which, button, .. } => {
                    let gamepad_id = GamepadId::from_u32(which as u32);
                    let button_state = ButtonState {
                        gamepad_id,
                        button
                    };
                    self.currently_pressed.insert(button_state);
                    self.just_pressed.insert(button_state);
                },
                Event::ControllerButtonUp { which, button, .. } => {
                    let gamepad_id = GamepadId::from_u32(which as u32);
                    let button_state = ButtonState {
                        gamepad_id,
                        button
                    };
                    self.currently_pressed.remove(&button_state);
                    self.just_released.insert(button_state);
                },
                Event::ControllerAxisMotion { .. } => {},
                _ => {}
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ButtonState {
    gamepad_id: GamepadId,
    button: sdl2::controller::Button,
}
