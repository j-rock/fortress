use app::StatusOr;
use gilrs::{
    EventType,
    Gilrs,
};

pub struct GamepadControls {
    game_input: Gilrs,
    // currently_pressed: HashSet<Scancode>,
    // just_pressed: HashSet<Scancode>,
    // just_released: HashSet<Scancode>,
}

impl GamepadControls {
    pub fn new() -> StatusOr<GamepadControls> {
        Ok(GamepadControls {
            game_input: Gilrs::new().map_err(|err| format!("Error creating controller input: {:?}", err))?
        })
    }

    pub fn update(&mut self) {
        while let Some(event) = self.game_input.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) => {},
                EventType::ButtonRepeated(button, _) => {},
                EventType::ButtonReleased(button, _) => {},
                EventType::ButtonChanged(button, value, _) => {},
                EventType::AxisChanged(axis, value32, _) => {},
                EventType::Connected => println!("Connected: {}", event.id),
                EventType::Disconnected => println!("Disconnected: {}", event.id),
                EventType::Dropped => {},
            }
            println!("New event from {}: {:?}", event.id, event.event);
        }
    }
}
