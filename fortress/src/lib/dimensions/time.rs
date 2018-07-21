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
        let nanos = duration.subsec_nanos() as Microseconds;
        1000000 * duration.as_secs() as i64 + nanos / 1000
    }

    pub fn new(duration: Duration) -> DeltaTime {
        DeltaTime {
            microseconds_elapsed: Self::duration_to_microseconds(duration)
        }
    }

    pub fn as_microseconds(&self) -> Microseconds {
        self.microseconds_elapsed
    }

    pub fn as_f32_seconds(&self) -> f32 {
        self.microseconds_elapsed as f32 / 1000000.0
    }
}
