use crate::consts::{Tetrimino, TETRIMINO_TYPES};
use crate::utils::shuffle;

// https://tetris.fandom.com/wiki/Random_Generator
pub struct Generator {
    seed: u64,
    bag: [Tetrimino; 7],
    next: i8,
}

impl Generator {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            bag: get_new_sequence_of_tetriminos(),
            next: 0,
        }
    }

    pub fn get_new_sequence_of_tetriminos(&mut self) -> [Tetrimino; 7] {
        get_new_sequence_of_tetriminos()
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

fn get_new_sequence_of_tetriminos() -> [Tetrimino; 7] {
    let mut bag = TETRIMINO_TYPES;
    shuffle(&mut bag);
    bag
}
