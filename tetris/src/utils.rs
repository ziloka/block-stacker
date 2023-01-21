// https://www.freecodecamp.org/news/random-number-generator
// true random number generators (TRNGs), pseudorandom number generators (PRNG)
// Linear Congruence method for generating Pseudo Random Numbers

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1871308106107e6b5abe96158e9ea82a
pub struct Random {
    r#mod: usize, // m > 0 (the modulus is positive)
    multiply: usize, // 0 < a < m (the multiplier is positive but less than the modulus)
    increment: usize, // 0 ≤ b < m (the increment is non negative but less than the modulus)
    pub seed: usize, // 0 ≤ X0 < m (the seed is non negative but less than the modulus)
}

impl Random {
    pub fn new(seed: usize) -> Self {
        Self {
            r#mod: 10,
            multiply: 7,
            increment: 5,
            seed: seed,
        }
    }

    pub fn next(&mut self) -> usize {
        let num = ((self.seed * self.multiply) + self.increment) % self.r#mod;
        self.seed = num;
        num
    }

    // generates an integer
    pub fn gen_range(&mut self, min: usize, max: usize) -> usize {
        let num = ((self.seed * self.multiply) + min) % max;
        self.seed = num;
        num
    }

    // https://www.geeksforgeeks.org/shuffle-a-given-array-using-fisher-yates-shuffle-algorithm/
    pub fn shuffle<T>(&mut self, arr: &mut [T; 7]) {
        for i in (1..arr.len()).rev() {
            let j = self.gen_range(0, i + 1);
            arr.swap(i, j);
        }
    }
    
}