use crate::collections::HugeLongArray;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub trait Random {
    fn next_int(&mut self, origin: i32, bound: i32) -> i32;
    fn next_long(&mut self, origin: usize, bound: usize) -> usize;
}

pub struct ShuffleUtil;

impl ShuffleUtil {
    pub fn shuffle_array(data: &mut HugeLongArray, random: &mut dyn Random) {
        let size = data.size();
        if size < 2 {
            return;
        }
        for offset in 0..(size - 1) {
            let swap_with = random.next_long(offset, size);
            let temp = data.get(swap_with);
            let current = data.get(offset);
            data.set(swap_with, current);
            data.set(offset, temp);
        }
    }

    pub fn shuffle_slice(data: &mut [i32], random: &mut dyn Random) {
        if data.len() < 2 {
            return;
        }
        for offset in 0..(data.len() - 1) {
            let swap_with = random.next_int(offset as i32, data.len() as i32) as usize;
            data.swap(offset, swap_with);
        }
    }

    pub fn create_random_data_generator(random_seed: Option<u64>) -> SplittableRandom {
        SplittableRandom::with_seed(random_seed)
    }
}

#[derive(Clone)]
pub struct SplittableRandom {
    seed: u64,
    gamma: u64,
}

impl SplittableRandom {
    const GOLDEN_GAMMA: u64 = 0x9e37_79b9_7f4a_7c15;

    pub fn new() -> Self {
        Self::with_seed(None)
    }

    pub fn with_seed(seed: Option<u64>) -> Self {
        let seed_value = seed.unwrap_or_else(Self::default_seed);
        let mixed_seed = Self::mix64(seed_value);
        let gamma = Self::mix_gamma(seed_value.wrapping_add(Self::GOLDEN_GAMMA));
        Self {
            seed: mixed_seed,
            gamma,
        }
    }

    pub fn split(&mut self) -> SplittableRandom {
        let new_seed = self.next64();
        SplittableRandom::with_seed(Some(new_seed))
    }

    fn default_seed() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_millis(0))
            .as_nanos() as u64
    }

    fn next_seed(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(self.gamma);
        self.seed
    }

    fn next64(&mut self) -> u64 {
        let mut z = self.next_seed();
        z ^= z >> 30;
        z = z.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z ^= z >> 27;
        z = z.wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn next32(&mut self) -> u32 {
        self.next64() as u32
    }

    fn mix64(seed: u64) -> u64 {
        let mut z = seed;
        z ^= z >> 30;
        z = z.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z ^= z >> 27;
        z = z.wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn mix_gamma(seed: u64) -> u64 {
        let z = Self::mix64(seed) | 1;
        let bit_count = (z ^ (z >> 1)).count_ones();
        if bit_count < 24 {
            z ^ 0xaaaa_aaaa_aaaa_aaaa
        } else {
            z
        }
    }
}

impl Random for SplittableRandom {
    fn next_int(&mut self, origin: i32, bound: i32) -> i32 {
        assert!(origin < bound, "bound must be greater than origin");
        let range = (bound - origin) as u32;
        let value = self.next32() % range;
        origin + value as i32
    }

    fn next_long(&mut self, origin: usize, bound: usize) -> usize {
        assert!(origin < bound, "bound must be greater than origin");
        let range = (bound - origin) as u64;
        let mut bits = self.next64();
        let result = if range.is_power_of_two() {
            bits & (range - 1)
        } else {
            let m = range - 1;
            loop {
                let u = bits >> 1;
                let candidate = u % range;
                let t = u.wrapping_add(m).wrapping_sub(candidate);
                if (t as i64) >= 0 {
                    break candidate;
                }
                bits = self.next64();
            }
        };
        origin + result as usize
    }
}

impl Default for SplittableRandom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_long_within_bounds() {
        let mut rng = SplittableRandom::with_seed(Some(1));
        for _ in 0..100 {
            let value = rng.next_long(0, 50);
            assert!(value < 50);
        }
    }

    #[test]
    fn next_int_within_bounds() {
        let mut rng = SplittableRandom::with_seed(Some(1234));
        for _ in 0..100 {
            let value = rng.next_int(10, 40);
            assert!(value >= 10 && value < 40);
        }
    }

    #[test]
    fn shuffle_slice_produces_permutation() {
        let mut data = vec![0, 1, 2, 3, 4, 5];
        let mut rng = SplittableRandom::with_seed(Some(42));
        ShuffleUtil::shuffle_slice(&mut data, &mut rng);
        let mut sorted = data.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn shuffle_array_produces_permutation() {
        let mut array = HugeLongArray::new(6);
        for i in 0..array.size() {
            array.set(i, i as i64);
        }
        let mut rng = SplittableRandom::with_seed(Some(99));
        ShuffleUtil::shuffle_array(&mut array, &mut rng);
        let mut sorted = array.to_vec();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5]);
    }
}
