use crate::{
    consts::{TetriminoType, BOARD},
    piece::Position,
};
use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct SelectedTetrimino {
    pub tetrimino_type: TetriminoType,
    pub position: Position,
}

#[derive(Component, Copy, Clone)]
pub struct Board {
    pub active_piece: SelectedTetrimino,
}