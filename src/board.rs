use macroquad::prelude::draw_rectangle;
use rand::seq::SliceRandom;
use std::time::SystemTime;

use crate::consts::{Piece, Position, BLOCK_SIZE, HEIGHT, TETRIMINO_TYPES, WIDTH};

pub struct Player {
    board: Board,
}

pub struct Board {
    pub left_top_corner: Position,
    pub right_bottom_corner: Position,
    pub time: SystemTime,
    pub active_piece: Piece,
    positions: [[u8; WIDTH as usize]; HEIGHT as usize],
}

impl Board {
    pub fn new(left_top_corner: Position, right_bottom_corner: Position) -> Self {
        Self {
            active_piece: Piece {
                tetrimino: TETRIMINO_TYPES
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
                position: Position {
                    x: left_top_corner.x,
                    y: left_top_corner.y,
                },
            },
            left_top_corner: left_top_corner,
            right_bottom_corner: right_bottom_corner,
            // https://stackoverflow.com/a/53930630
            positions: [[0; WIDTH as usize]; HEIGHT as usize],
            time: SystemTime::now(),
        }
    }

    pub fn new_tetrimino(&mut self) {
        let tetrimino = TETRIMINO_TYPES
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        self.active_piece = Piece {
            tetrimino: tetrimino,
            position: Position {
                x: self.left_top_corner.x,
                y: self.left_top_corner.y,
            },
        };
    }

    pub fn draw_tetrimino(&self) {
        // draw current block
        for position in self.active_piece.tetrimino.get_structure() {
            draw_rectangle(
                self.active_piece.position.x + position.x * BLOCK_SIZE,
                self.active_piece.position.y + position.y * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                self.active_piece.tetrimino.get_color(),
            );
        }
    }

    pub fn rotate_tetrimino(&self) {
      
    }

    pub fn conflict(&self, x: f32, y: f32) -> bool {
        if x < self.left_top_corner.x // for the left side
        || x > self.right_bottom_corner.x // for the right side
        || y > self.right_bottom_corner.y
        // for the floor (bottom)
        {
            true
        } else {
            false
        }
    }
}
