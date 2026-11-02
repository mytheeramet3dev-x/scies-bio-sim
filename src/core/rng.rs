//! Seedable, deterministic pseudo-random number generator for biological simulations.

use serde::{Deserialize, Serialize};

/// High-performance, deterministic PRNG based on SplitMix64 & XorShift64*.
/// Ensures bit-for-bit identical biological outcomes for identical initial seeds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicRng {
    initial_seed: u64,
    state: u64,
}

impl DeterministicRng {
    /// Creates a new deterministic RNG initialized with the given seed.
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            initial_seed: seed,
            state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed },
        };
        // Warm up state to decorrelate low seed numbers
        for _ in 0..8 {
            rng.next_u64();
        }
        rng
    }

    /// Returns the initial seed.
    pub fn seed(&self) -> u64 {
        self.initial_seed
    }

    /// Resets the generator back to its initial seeded state.
    pub fn reset(&mut self) {
        *self = Self::new(self.initial_seed);
    }

    /// Generates next 64-bit pseudo-random unsigned integer using SplitMix64.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    /// Generates a floating-point number in `[0.0, 1.0)`.
    pub fn gen_f64(&mut self) -> f64 {
        let bits = self.next_u64() >> 11;
        (bits as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    /// Generates a floating-point number in range `[min, max]`.
    pub fn gen_range_f64(&mut self, min: f64, max: f64) -> f64 {
        if min >= max {
            return min;
        }
        min + self.gen_f64() * (max - min)
    }

    /// Generates an integer in range `[min, max]`.
    pub fn gen_range_u64(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        let range = max - min + 1;
        min + (self.next_u64() % range)
    }

    /// Evaluates a Bernoulli trial with probability `p` of returning true.
    pub fn gen_bool(&mut self, p: f64) -> bool {
        if p <= 0.0 {
            false
        } else if p >= 1.0 {
            true
        } else {
            self.gen_f64() < p
        }
    }

    /// Generates a normally distributed value using Box-Muller transform.
    pub fn gen_normal(&mut self, mean: f64, std_dev: f64) -> f64 {
        let u1 = self.gen_f64().max(1e-15);
        let u2 = self.gen_f64();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        mean + z0 * std_dev
    }

    /// In-place Fisher-Yates slice shuffle.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }
        for i in (1..len).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let mut rng1 = DeterministicRng::new(42);
        let mut rng2 = DeterministicRng::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
            assert_eq!(rng1.gen_f64(), rng2.gen_f64());
        }
    }

    #[test]
    fn test_bernoulli() {
        let mut rng = DeterministicRng::new(12345);
        let mut true_count = 0;
        let trials = 10_000;
        for _ in 0..trials {
            if rng.gen_bool(0.7) {
                true_count += 1;
            }
        }
        let ratio = true_count as f64 / trials as f64;
        assert!((ratio - 0.7).abs() < 0.02);
    }
}
