use control::{
    ControlEvent,
    ControllerEvent,
    GamepadId
};
use dimensions::LrDirection;
use sdl2::{
    controller::GameController,
    event::Event,
    self
};
use std::collections::{
    HashMap,
    HashSet,
};

pub struct GamepadControls {
    controller_events: Vec<ControllerEvent>,
    gamepads: HashMap<GamepadId, GameController>,
    axes: HashMap<GamepadAxis, f32>,
    currently_pressed: HashSet<GamepadButton>,
    just_pressed: HashSet<GamepadButton>,
    just_released: HashSet<GamepadButton>,
}

impl GamepadControls {
    pub fn new() -> GamepadControls {
        GamepadControls {
            controller_events: Vec::new(),
            gamepads: HashMap::new(),
            axes: HashMap::new(),
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_pressed(&self, gamepad_id: GamepadId, event: ControlEvent) -> bool {
        let gamepad_button = Self::control_event_to_button(gamepad_id, event);
        self.currently_pressed.contains(&gamepad_button)
    }

    pub fn just_pressed(&self, gamepad_id: GamepadId, event: ControlEvent) -> bool {
        let gamepad_button = Self::control_event_to_button(gamepad_id, event);
        self.just_pressed.contains(&gamepad_button)
    }

    pub fn just_released(&self, gamepad_id: GamepadId, event: ControlEvent) -> bool {
        let gamepad_button = Self::control_event_to_button(gamepad_id, event);
        self.just_released.contains(&gamepad_button)
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
                    let gamepad_button = GamepadButton {
                        gamepad_id,
                        button
                    };
                    self.currently_pressed.insert(gamepad_button);
                    self.just_pressed.insert(gamepad_button);
                },
                Event::ControllerButtonUp { which, button, .. } => {
                    let gamepad_id = GamepadId::from_u32(which as u32);
                    let gamepad_button = GamepadButton {
                        gamepad_id,
                        button
                    };
                    self.currently_pressed.remove(&gamepad_button);
                    self.just_released.insert(gamepad_button);
                },
                Event::ControllerAxisMotion { which, axis, value, .. } => {
                    let gamepad_id = GamepadId::from_u32(which as u32);
                    let gamepad_axis = GamepadAxis {
                        gamepad_id,
                        axis
                    };
                    let normalized_value = value as f32  / i16::max_value() as f32;
                    self.axes.insert(gamepad_axis, normalized_value);
                }
                _ => {}
            }
        }
        println!("{:?}", self.currently_pressed);
    }

    fn control_event_to_button(gamepad_id: GamepadId, event: ControlEvent) -> GamepadButton {
        let button = match event {
            ControlEvent::PlayerFire => sdl2::controller::Button::RightShoulder,
            ControlEvent::PlayerJump => sdl2::controller::Button::A,
            ControlEvent::PlayerMove(LrDirection::Left) => sdl2::controller::Button::DPadLeft,
            ControlEvent::PlayerMove(LrDirection::Right) => sdl2::controller::Button::DPadRight,
            ControlEvent::PlayerSlash => sdl2::controller::Button::X,
            ControlEvent::RespawnEntities => sdl2::controller::Button::Back,
        };
        GamepadButton {
            gamepad_id,
            button
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GamepadButton {
    gamepad_id: GamepadId,
    button: sdl2::controller::Button,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GamepadAxis {
    gamepad_id: GamepadId,
    axis: sdl2::controller::Axis,
}
