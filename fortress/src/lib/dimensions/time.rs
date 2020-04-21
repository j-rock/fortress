use std::time::Duration;

pub type Microseconds = i64;

pub fn milliseconds(t: i64) -> Microseconds {
    t * 1000
}

#[derive(Copy, Clone)]
pub struct DeltaTime {
    microseconds_elapsed: Microseconds
}

impl DeltaTime {
    fn duration_to_microseconds(duration: Duration) -> Microseconds {
        let nanos = Microseconds::from(duration.subsec_nanos());
        1_000_000 * duration.as_secs() as i64 + nanos / 1000
    }

    pub fn new(duration: Duration) -> DeltaTime {
        DeltaTime {
            microseconds_elapsed: Self::duration_to_microseconds(duration)
        }
    }

    pub fn as_microseconds(self) -> Microseconds {
        self.microseconds_elapsed
    }

    pub fn as_f32_seconds(self) -> f32 {
        self.microseconds_elapsed as f32 / 1_000_000.0
    }

    pub fn as_f64_seconds(self) -> f64 {
        self.microseconds_elapsed as f64 / 1_000_000.0
    }
}

pub struct Timer {
    time_left: Microseconds,
}

impl Timer {
    pub fn expired() -> Self {
        Timer {
            time_left: 0
        }
    }

    pub fn new(duration: Microseconds) -> Self {
        Timer {
            time_left: duration
        }
    }

    pub fn tick(&mut self, dt: DeltaTime) {
        self.time_left -= dt.as_microseconds();
        if self.time_left < 0 {
            self.time_left = 0;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.time_left == 0
    }

    pub fn time_left(&self) -> Microseconds {
        self.time_left
    }

    pub fn as_completion_fraction_of(&self, duration: Microseconds) -> f32 {
        1.0 - self.time_left as f32 / duration as f32
    }
}
