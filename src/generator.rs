use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::consts::{TETRIMINO_TYPES, Tetrimino};

// https://tetris.fandom.com/wiki/Random_Generator
pub struct Generator {
    seed: u64,
    rng: ChaCha20Rng,
    bag: [Tetrimino; 7],
    next: i8
}

impl Generator {
  pub fn new(seed: u64) -> Self {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    Self {
      seed,
      bag: get_new_sequence_of_tetriminos(&mut rng),
      rng: rng,
      next: 0
    }
  }

  pub fn get_new_sequence_of_tetriminos(&mut self) -> [Tetrimino; 7] {
    get_new_sequence_of_tetriminos(&mut self.rng)
  }

  pub fn next(&mut self) -> Tetrimino {
    if self.next == 7 {
      self.bag = self.get_new_sequence_of_tetriminos();
      self.next = 0;
    }

    let tetrimino = self.bag[self.next as usize];
    self.next += 1;

    tetrimino
  }
}

fn get_new_sequence_of_tetriminos(rng: &mut ChaCha20Rng) -> [Tetrimino; 7] {
  let mut bag = TETRIMINO_TYPES.clone();

  bag.shuffle( rng);

  bag
}