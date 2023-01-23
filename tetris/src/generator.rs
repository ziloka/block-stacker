use crate::consts::{Tetromino, TETROMINO_TYPES};
use crate::utils::Random;

// https://tetris.fandom.com/wiki/Random_Generator
pub struct Generator {
    random: Random,
    bag: [Tetromino; 7],
    index: usize,
}

impl Generator {
    pub fn new(seed: usize) -> Self {
        let mut random = Random::new(seed);
        let mut bag = TETROMINO_TYPES;
        random.shuffle(&mut bag);
        Self {
            random,
            bag,
            index: 0,
        }
    }

    pub fn get_new_sequence_of_tetrominos(&mut self) -> [Tetromino; 7] {
        let mut bag = TETROMINO_TYPES;
        self.random.shuffle(&mut bag);
        bag
    }
}

impl Iterator for Generator {
    type Item = Tetromino;

    fn next(&mut self) -> Option<Tetromino> {
        let res = if let Some(tetromino) = self.bag.get(self.index).copied() {
            tetromino
        } else {
            self.index = 0;
            self.bag = self.get_new_sequence_of_tetrominos();
            self.bag[self.index]
        };
        self.index += 1;
        Some(res)
    }
}

