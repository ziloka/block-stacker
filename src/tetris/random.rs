use std::ops::Range;

// implement pcg xsh rr (Xorshift High/Retry)
#[derive(Clone)]
pub struct Random {
    state: u64,
    inc: u64,
}

impl Random {
    // The stream variable in the PCG XSH RR generator is used to create different sequences of random numbers from the same seed.
    // By providing a different stream value, it allows the generator to produce distinct sequences of random numbers, which can be useful in situations where multiple independent PRNGs are needed.
    // It is essentially a way to shift the starting point of the generated sequence of numbers, so that it does not overlap with other generated sequences that may have been created with the same seed.
    // if you want to achieve this feature, add the stream parameter as the second argument to the constructor, and add it to the state and inc fields.
    pub fn new(seed: u64) -> Self {
        Self {
            state: 0x853c49e6748fea9b + seed,
            inc: 0xda3e39cb94b95bdb + seed,
        }
    }

    fn next(&mut self) -> usize {
        let oldstate = self.state;
        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);
        let xorshifted = ((oldstate >> 18) ^ oldstate) >> 27;
        let rot = oldstate >> 59;
        (xorshifted as usize) >> rot | (xorshifted as usize) << ((-(rot as i64)) & 31)
    }

    // // https://www.geeksforgeeks.org/shuffle-a-given-array-using-fisher-yates-shuffle-algorithm/
    pub fn shuffle<T>(&mut self, arr: &mut [T; 7]) {
        for i in 1..arr.len() {
            arr.swap(i, self.gen_range(0..i + 1));
        }
    }

    fn gen_range(&mut self, range: Range<usize>) -> usize {
        let range_size = range.end - range.start;
        let threshold = (std::usize::MAX - range_size) % range_size;
        let mut r = self.next();
        while r < threshold {
            r = self.next();
        }
        range.start + (r % range_size)
    }
}
