use control::{
    Controller,
    ControlEvent::PlayerSlash
};
use dimensions::{
    time::{
        DeltaTime,
        self,
    }
};
use player::{
    PlayerConfig,
    PlayerState,
};

#[derive(Copy, Clone)]
struct CurrentSlash {
    time_left: time::Microseconds,
}

impl CurrentSlash {
    pub fn new(time_left: time::Microseconds) -> CurrentSlash {
        CurrentSlash {
            time_left
        }
    }
}

#[derive(Copy, Clone)]
pub struct SlashState {
    slash_period: time::Microseconds,
    current_slash: Option<CurrentSlash>,
}

impl SlashState {
    pub fn new(config: &PlayerConfig) -> SlashState {
        SlashState {
            slash_period: time::milliseconds(config.slash_period_ms),
            current_slash: None,
        }
    }

    pub fn update(&mut self, player_state: &mut PlayerState, controller: &Controller, dt: DeltaTime) {
        self.try_elapsing_current_slash(player_state, dt);

        if controller.just_pressed(PlayerSlash) {
            self.try_slash(player_state);
        }
    }

    fn try_elapsing_current_slash(&mut self, player_state: &mut PlayerState, dt: DeltaTime) {
        if let Some(mut current_slash) = self.current_slash {
            current_slash.time_left -= dt.as_microseconds();

            if current_slash.time_left > 0 {
                self.current_slash = Some(current_slash);
                return;
            }

            player_state.body.disable_sword_collision();
            self.current_slash = None;
        }
    }

    fn try_slash(&mut self, player_state: &mut PlayerState) {
        if let None = self.current_slash {
            player_state.body.enable_sword_collision();
            self.current_slash = Some(CurrentSlash::new(self.slash_period));
        }
    }
}