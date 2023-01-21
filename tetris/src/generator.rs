use crate::consts::{Tetrimino, TETRIMINO_TYPES};
use crate::utils::Random;

// https://tetris.fandom.com/wiki/Random_Generator
pub struct Generator {
    random: Random,
    bag: [Tetrimino; 7],
    next: usize,
}

impl Generator {
    pub fn new(seed: usize) -> Self {
        let mut random = Random::new(seed);
        let mut bag = TETRIMINO_TYPES;
        random.shuffle(&mut bag);
        Self {
            random: random,
            bag: bag,
            next: 0,
        }
    }

    pub fn get_new_sequence_of_tetriminos(&mut self) -> [Tetrimino; 7] {
        let mut bag = TETRIMINO_TYPES;
        self.random.shuffle(&mut bag);
        bag
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

