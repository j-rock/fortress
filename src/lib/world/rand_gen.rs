use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::{
    self,
    time::SystemTime
};

pub struct RandGen {
    pub rng: XorShiftRng,
    _seed: [u8; 16],
}

impl Default for RandGen {
    fn default() -> Self {
        Self::new()
    }
}

impl RandGen {
    pub fn new() -> RandGen {
        let seed = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs();
                let nanos = duration.subsec_nanos();
                [secs, u64::from(nanos)]

            },
            Err(_) => {
                [0, 0]
            }
        };
        let seed: [u8; 16] = unsafe { std::mem::transmute(seed) };
        RandGen {
            rng: XorShiftRng::from_seed(seed),
            _seed: seed
        }
    }
}