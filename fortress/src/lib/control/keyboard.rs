use control::ControlEvent;
use dimensions::LrDirection;
use sdl2::{
    EventPump,
    keyboard::Scancode
};
use std::collections::HashSet;

pub struct KeyboardControls {
    first_time_used: FirstTimeUsed,
    currently_pressed: HashSet<Scancode>,
    just_pressed: HashSet<Scancode>,
    just_released: HashSet<Scancode>,
}

impl Default for KeyboardControls {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardControls {
    pub fn new() -> KeyboardControls {
        KeyboardControls {
            first_time_used: FirstTimeUsed::new(),
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn update(&mut self, e: &EventPump) {
        let currently_pressed: HashSet<_> = e.keyboard_state().pressed_scancodes().collect();
        self.first_time_used.touch(!currently_pressed.is_empty());
        self.just_pressed.clear();
        for scancode in currently_pressed.difference(&self.currently_pressed) {
            self.just_pressed.insert(scancode.clone());
        }

        self.just_released.clear();
        for scancode in self.currently_pressed.difference(&currently_pressed) {
            self.just_released.insert(scancode.clone());
        }

        self.currently_pressed = currently_pressed;
    }

    pub fn is_pressed(&self, event: ControlEvent) -> bool {
        let scancode = Self::control_event_to_scancode(event);
        self.currently_pressed.contains(&scancode)
    }

    pub fn just_pressed(&self, event: ControlEvent) -> bool {
        let scancode = Self::control_event_to_scancode(event);
        self.just_pressed.contains(&scancode)
    }

    pub fn just_released(&self, event: ControlEvent) -> bool {
        let scancode = Self::control_event_to_scancode(event);
        self.just_released.contains(&scancode)
    }

    pub fn just_joined(&self) -> bool {
        self.first_time_used.is_first()
    }

    fn control_event_to_scancode(event: ControlEvent) -> Scancode {
        match event {
            ControlEvent::PlayerFire => Scancode::I,
            ControlEvent::PlayerJump => Scancode::Space,
            ControlEvent::PlayerMove(LrDirection::Left) => Scancode::A,
            ControlEvent::PlayerMove(LrDirection::Right) => Scancode::D,
            ControlEvent::PlayerSlash => Scancode::J,
            ControlEvent::RedeployEntities => Scancode::R,
        }
    }
}

struct FirstTimeUsed {
    pub has_been_used: bool,
    pub first_time_used: bool,
}

impl FirstTimeUsed {
    pub fn new() -> FirstTimeUsed {
        FirstTimeUsed {
            has_been_used: false,
            first_time_used: false,
        }
    }

    pub fn touch(&mut self, cond: bool) {
        if cond {
            self.first_time_used = !self.has_been_used;
            self.has_been_used = true;
        }
    }

    pub fn is_first(&self) -> bool {
        self.first_time_used
    }
}
