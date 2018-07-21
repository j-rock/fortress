use sdl2::{
    EventPump,
    keyboard::Scancode
};
use std::collections::HashSet;

pub struct KeyboardControls {
    currently_pressed: HashSet<Scancode>,
    just_pressed: HashSet<Scancode>,
    just_released: HashSet<Scancode>,
}

impl KeyboardControls {
    pub fn new() -> KeyboardControls {
        KeyboardControls {
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn update(&mut self, e: &EventPump) {
        let currently_pressed: HashSet<_> = e.keyboard_state().pressed_scancodes().collect();
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

    pub fn is_pressed(&self, scancode: Scancode) -> bool {
        self.currently_pressed.contains(&scancode)
    }

    pub fn just_pressed(&self, scancode: Scancode) -> bool {
        self.just_pressed.contains(&scancode)
    }

    pub fn just_released(&self, scancode: Scancode) -> bool {
        self.just_released.contains(&scancode)
    }
}