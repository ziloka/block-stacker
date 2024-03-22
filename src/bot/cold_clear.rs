use crate::game::Player;
use cold_clear_2::{data::{Board, Piece, Placement}, movegen::find_moves};

use crate::core::logic::consts::Tetromino as CorePiece;
use Piece as ColdClearPiece;

struct ColdClear {
    board: Board,
    current_piece: Piece
}

impl Player for ColdClear {
    fn do_work(&self) {

    }

    fn on_next_piece(&mut self, tetromino: CorePiece) {
        self.current_piece = utils::convert_piece(tetromino);
    }
}

impl ColdClear {

    pub fn new(tetromino: CorePiece) -> Self {
        Self {
            board: Board {
                cols: [0; 10]
            },
            current_piece: utils::convert_piece(tetromino)
        }
    }

    // pub fn update(&mut self, board: Board) {
    //     self.board = board;
    // }

    pub fn get_next_move(&self) -> Vec<(Placement, u32)> {
        find_moves(&self.board, self.current_piece)
    }
}

mod utils {
    pub fn convert_piece(tetromino: super::CorePiece) -> super::ColdClearPiece {
        match tetromino {
            crate::core::logic::consts::Tetromino::I => super::ColdClearPiece::I,
            crate::core::logic::consts::Tetromino::O => super::ColdClearPiece::O,
            crate::core::logic::consts::Tetromino::T => super::ColdClearPiece::T,
            crate::core::logic::consts::Tetromino::S => super::ColdClearPiece::S,
            crate::core::logic::consts::Tetromino::Z => super::ColdClearPiece::Z,
            crate::core::logic::consts::Tetromino::J => super::ColdClearPiece::J,
            crate::core::logic::consts::Tetromino::L => super::ColdClearPiece::L,
        }
    }
}