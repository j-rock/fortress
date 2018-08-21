use dimensions::{
    Damage,
    time::{
        DeltaTime,
        self,
    }
};
use player::{
    PlayerConfig,
    state::PlayerBody,
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
    sword_knockback_strength: f32,
    sword_damage: Damage,
    slash_period: time::Microseconds,
    current_slash: Option<CurrentSlash>,
}

impl SlashState {
    pub fn new(config: &PlayerConfig) -> SlashState {
        SlashState {
            slash_period: time::milliseconds(config.slash_period_ms),
            sword_damage: config.sword_damage,
            sword_knockback_strength: config.sword_knockback_strength,
            current_slash: None,
        }
    }

    pub fn pre_update(&mut self, player_body: &mut PlayerBody, dt: DeltaTime) {
        if let Some(mut current_slash) = self.current_slash {
            current_slash.time_left -= dt.as_microseconds();

            if current_slash.time_left > 0 {
                self.current_slash = Some(current_slash);
                return;
            }

            player_body.disable_sword_collision();
            self.current_slash = None;
        }
    }

    pub fn try_slash(&mut self, body: &mut PlayerBody) {
        if let None = self.current_slash {
            body.enable_sword_collision();
            self.current_slash = Some(CurrentSlash::new(self.slash_period));
        }
    }

    pub fn get_sword_knockback_strength(&self) -> f32 {
        self.sword_knockback_strength
    }

    pub fn get_sword_damage(&self) -> Damage {
        self.sword_damage
    }
}