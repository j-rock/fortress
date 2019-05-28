use glm;
use rand::{
    Rng,
    SeedableRng,
    distributions::{
        Distribution,
        UnitCircle,
    },
};
use rand_xorshift::XorShiftRng;
use std::{
    self,
    time::SystemTime
};

pub struct RandGen {
    pub rng: XorShiftRng,
    unit_circle: UnitCircle,
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
            unit_circle: UnitCircle::new(),
            _seed: seed
        }
    }

    pub fn unit_circle_glm(&mut self) -> glm::Vec2 {
        let [x, y] = self.unit_circle.sample(&mut self.rng);
        glm::vec2(x as f32, y as f32)
    }

    pub fn unit_f32(&mut self) -> f32 {
        self.rng.gen_range(0.0, 1.0)
    }
}