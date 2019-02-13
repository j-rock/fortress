use crate::dimensions::time::DeltaTime;
use std::time::Instant;

pub struct Clock {
    last_instant: Instant
}

impl Clock {
    pub fn start() -> Clock {
        Clock {
            last_instant: Instant::now()
        }
    }
    pub fn restart(&mut self) -> DeltaTime {
        let new_instant = Instant::now();
        let duration = new_instant.duration_since(self.last_instant);
        self.last_instant = new_instant;
        DeltaTime::new(duration)
    }
}
