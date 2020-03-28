use glm;

pub struct NoiseFn;

static PRIMES: &'static [f32] = &[2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0, 19.0, 23.0];

impl NoiseFn {
    // Returns value in range [0.0, 1.0].
    pub fn custom(seed: f32, t: f32, iterations: usize) -> f32 {
        let noise = |v: glm::Vec2| v.x.sin() * v.y.cos();

        let mut p = glm::vec2(seed, t);
        let mut f = 0.0;
        let largest_weight = 1 << iterations;
        let mut weight = largest_weight >> 1;
        for i in 0..iterations {
            f += (weight as f32) * noise(p);
            p = p * PRIMES[i % PRIMES.len()];
            weight = weight >> 1;
        }
        f += largest_weight as f32 * noise(glm::vec2(f, noise(p)));
        f /= (2 * largest_weight - 1) as f32;
        f = 0.5 * (f + 1.0);
        return f;
    }
}