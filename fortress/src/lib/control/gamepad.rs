use control::{
    ControlEvent,
    ControllerEvent,
    GamepadConfig,
    GamepadId,
};
use dimensions::LrDirection;
use file::SimpleConfigManager;
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
    config_manager: SimpleConfigManager<GamepadConfig>,
    controller_events: Vec<ControllerEvent>,
    gamepads: HashMap<GamepadId, GameController>,
    axes: HashMap<GamepadAxis, f32>,
    currently_pressed: HashSet<GamepadButton>,
    just_pressed: HashSet<GamepadButton>,
    just_released: HashSet<GamepadButton>,
}

impl GamepadControls {
    pub fn new(config_manager: SimpleConfigManager<GamepadConfig>) -> GamepadControls {
        GamepadControls {
            config_manager,
            controller_events: Vec::new(),
            gamepads: HashMap::new(),
            axes: HashMap::new(),
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_pressed(&self, gamepad_id: GamepadId, event: ControlEvent) -> bool {
        match self.control_event_to_gamepad_control(event) {
            GamepadControl::ButtonPress(button) => {
                let gamepad_button = GamepadButton {
                    gamepad_id,
                    button
                };
                self.currently_pressed.contains(&gamepad_button)
            },
            GamepadControl::AxisAboveThreshold(axis, thresh) => {
                let gamepad_axis = GamepadAxis {
                    gamepad_id,
                    axis
                };
                if let Some(value) = self.axes.get(&gamepad_axis) {
                    *value > thresh
                } else {
                    false
                }
            },
            GamepadControl::AxisBelowThreshold(axis, thresh) => {
                let gamepad_axis = GamepadAxis {
                    gamepad_id,
                    axis
                };
                if let Some(value) = self.axes.get(&gamepad_axis) {
                    *value < thresh
                } else {
                    false
                }
            }
        }
    }

    pub fn just_pressed(&self, gamepad_id: GamepadId, event: ControlEvent) -> bool {
        match self.control_event_to_gamepad_control(event) {
            GamepadControl::ButtonPress(button) => {
                let gamepad_button = GamepadButton {
                    gamepad_id,
                    button
                };
                self.just_pressed.contains(&gamepad_button)
            },
            _ => false
        }
    }

    pub fn just_released(&self, gamepad_id: GamepadId, event: ControlEvent) -> bool {
        match self.control_event_to_gamepad_control(event) {
            GamepadControl::ButtonPress(button) => {
                let gamepad_button = GamepadButton {
                    gamepad_id,
                    button
                };
                self.just_released.contains(&gamepad_button)
            },
            _ => false
        }
    }

    pub fn controller_events(&self) -> &Vec<ControllerEvent> {
        &self.controller_events
    }

    pub fn ingest_gamepad_events(&mut self, controller_subsystem: &sdl2::GameControllerSubsystem, gamepad_events: Vec<Event>) {
        self.config_manager.update();
        self.controller_events.clear();
        self.just_released.clear();
        self.just_pressed.clear();

        for event in gamepad_events.into_iter() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(game_controller) = controller_subsystem.open(which) {
                        let gamepad_id = GamepadId::from_i32(game_controller.instance_id());
                        self.gamepads.insert(gamepad_id, game_controller);
                        self.controller_events.push(ControllerEvent::GamepadConnected(gamepad_id));
                    } else {
                        println!("Couldn't open gamepad {}", which);
                    }
                },
                Event::ControllerDeviceRemoved { which, .. } => {
                    let gamepad_id = GamepadId::from_i32(which);
                    self.gamepads.remove(&gamepad_id);
                    self.controller_events.push(ControllerEvent::GamepadDisconnected(gamepad_id));
                },
                Event::ControllerButtonDown { which, button, .. } => {
                    let gamepad_id = GamepadId::from_i32(which);
                    let gamepad_button = GamepadButton {
                        gamepad_id,
                        button
                    };
                    self.currently_pressed.insert(gamepad_button);
                    self.just_pressed.insert(gamepad_button);
                },
                Event::ControllerButtonUp { which, button, .. } => {
                    let gamepad_id = GamepadId::from_i32(which);
                    let gamepad_button = GamepadButton {
                        gamepad_id,
                        button
                    };
                    self.currently_pressed.remove(&gamepad_button);
                    self.just_released.insert(gamepad_button);
                },
                Event::ControllerAxisMotion { which, axis, value, .. } => {
                    let gamepad_id = GamepadId::from_i32(which);
                    let gamepad_axis = GamepadAxis {
                        gamepad_id,
                        axis
                    };
                    let normalized_value = f32::from(value) / f32::from(i16::max_value());
                    self.axes.insert(gamepad_axis, normalized_value);
                }
                _ => {}
            }
        }
    }

    fn control_event_to_gamepad_control(&self, event: ControlEvent) -> GamepadControl {
        let config = self.config_manager.get();
        match event {
            ControlEvent::PlayerFire => GamepadControl::AxisAboveThreshold(sdl2::controller::Axis::TriggerRight, config.axis_threshold),
            ControlEvent::PlayerJump => GamepadControl::ButtonPress(sdl2::controller::Button::A),
            ControlEvent::PlayerMove(LrDirection::Left) => GamepadControl::AxisBelowThreshold(sdl2::controller::Axis::LeftX, -config.axis_threshold),
            ControlEvent::PlayerMove(LrDirection::Right) => GamepadControl::AxisAboveThreshold(sdl2::controller::Axis::LeftX, config.axis_threshold),
            ControlEvent::PlayerSlash => GamepadControl::ButtonPress(sdl2::controller::Button::X),
            ControlEvent::RedeployEntities => GamepadControl::ButtonPress(sdl2::controller::Button::Back),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GamepadButton {
    gamepad_id: GamepadId,
    button: sdl2::controller::Button,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GamepadAxis {
    gamepad_id: GamepadId,
    axis: sdl2::controller::Axis,
}

enum GamepadControl {
    ButtonPress(sdl2::controller::Button),
    AxisAboveThreshold(sdl2::controller::Axis, f32),
    AxisBelowThreshold(sdl2::controller::Axis, f32),
}

