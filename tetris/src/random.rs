use std::ops::{Bound, RangeBounds};

// https://www.freecodecamp.org/news/random-number-generator
// true random number generators (TRNGs), pseudorandom number generators (PRNG)
// Linear Congruence method for generating Pseudo Random Numbers

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36592888875f26e22e132e1ad164433c
pub struct Random {
    modulus: u64, // m > 0 (the modulus is positive)
    multiply: u64, // 0 < a < m (the multiplier is positive but less than the modulus)
    increment: u64, // 0 ≤ b < m (the increment is non negative but less than the modulus)
    pub seed: u64, // 0 ≤ X0 < m (the seed is non negative but less than the modulus)
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            modulus: 10,
            multiply: 7,
            increment: 5,
            seed,
        }
    }

    // pub fn next(&mut self) -> usize {
    //     let num = ((self.seed * self.multiply) + self.increment) % self.modulus;
    //     self.seed = num;
    //     num
    // }

    // // generates an integer [min, max), 
    // // https://www.pcg-random.org/posts/bounded-rands.html
    // pub fn gen_range(&mut self, min: usize, max: usize) -> usize {
    //     let modulus = max;
    //     let multiply = modulus - 1;
    //     let num = ((self.seed * multiply) + min) % modulus;
    //     println!("(({} * {}) + {}) % {} = {}", self.seed, multiply, min, modulus, num);
    //     self.seed = num;
    //     num
    // }

    // // https://www.geeksforgeeks.org/shuffle-a-given-array-using-fisher-yates-shuffle-algorithm/
    pub fn shuffle<T>(&mut self, arr: &mut [T; 7]) {
        for i in 1..arr.len() {
            arr.swap(i, self.usize(..i+1));
        }
    }

    pub fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        let panic_empty_range = || {
            panic!(
                "empty range: {:?}..{:?}",
                range.start_bound(),
                range.end_bound()
            )
        };
    
        let low = match range.start_bound() {
            Bound::Unbounded => std::usize::MIN,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x.checked_add(1).unwrap_or_else(panic_empty_range),
        };
    
        let high = match range.end_bound() {
            Bound::Unbounded => std::usize::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x.checked_sub(1).unwrap_or_else(panic_empty_range),
        };
    
        if low > high {
            panic_empty_range();
        }
    
        if low == std::usize::MIN && high == std::usize::MAX {
            self.gen_u64() as usize
        } else {
            let len = high.wrapping_sub(low).wrapping_add(1);
            low.wrapping_add(self.gen_mod_u64(len as usize as _) as usize)
        }
    }
    
    fn gen_u64(&mut self) -> u64 {
        let s = self.seed.wrapping_add(0xA0761D6478BD642F);
        self.seed = s;
        let t = u128::from(s) * u128::from(s ^ 0xE7037ED1A0B428DB);
        (t as u64) ^ (t >> 64) as u64
    }
    
    fn gen_mod_u64(&mut self, n: u64) -> u64 {
        let mut r = self.gen_u64();
        let mut hi = mul_high_u64(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u64();
                hi = mul_high_u64(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }

}

#[inline]
fn mul_high_u64(a: u64, b: u64) -> u64 {
    (((a as u128) * (b as u128)) >> 64) as u64
}