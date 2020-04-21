use glm;
use nalgebra::Point2;
use rand::{
    Rng,
    SeedableRng,
};
use rand_distr::{
    Distribution,
    UnitCircle,
};
use rand_xorshift::XorShiftRng;
use std::{
    self,
    time::SystemTime
};

pub struct RandGen {
    rng: XorShiftRng,
    _seed: [u8; 16],
}

impl Default for RandGen {
    fn default() -> Self {
        Self::new()
    }
}

impl RandGen {
    pub fn new() -> Self {
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

    pub fn unit_circle_glm(&mut self) -> glm::Vec2 {
        let [x, y]: [f32; 2] = UnitCircle.sample(&mut self.rng);
        glm::vec2(x, y)
    }

    pub fn unit_circle_point(&mut self) -> Point2<f64> {
        let [x, y]: [f64; 2] = UnitCircle.sample(&mut self.rng);
        Point2::new(x, y)
    }

    pub fn unit_f32(&mut self) -> f32 {
        self.ranged_f32(0.0, 1.0)
    }

    pub fn ranged_f32(&mut self, low: f32, high: f32) -> f32 {
        self.rng.gen_range(low, high)
    }

    pub fn unit_f64(&mut self) -> f64 {
        self.ranged_f64(0.0, 1.0)
    }

    pub fn ranged_f64(&mut self, low: f64, high: f64) -> f64 {
        self.rng.gen_range(low, high)
    }
}