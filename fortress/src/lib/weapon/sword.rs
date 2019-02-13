use audio::{
    AudioPlayer,
    Sound
};
use dimensions::{
    Damage,
    time::{
        DeltaTime,
        self,
    }
};
use players::{
    PlayerConfig,
    state::PlayerBody,
};
use weapon::SwordStats;

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
pub struct Sword {
    pub stats: SwordStats,
    current_slash: Option<CurrentSlash>,
}

impl Sword {
    pub fn new(config: &PlayerConfig) -> Sword {
        Sword {
            stats: SwordStats::new(config),
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

    pub fn try_slash(&mut self, body: &mut PlayerBody, audio: &AudioPlayer) {
        if self.current_slash.is_none() {
            body.enable_sword_collision();
            self.current_slash = Some(CurrentSlash::new(self.stats.get_period()));

            audio.play_sound(Sound::Slash);
        }
    }

    pub fn get_knockback_strength(&self) -> f32 {
        self.stats.get_knockback_strength()
    }

    pub fn get_damage(&self) -> Damage {
        self.stats.get_damage()
    }
}