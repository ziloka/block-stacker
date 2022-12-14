use crate::consts::TetriminoType;
use bevy::prelude::{Bundle, Component};

#[derive(Component, Copy, Clone)]
pub struct SelectedTetrimino {
    pub tetrimino_type: TetriminoType
}

#[derive(Bundle, Copy, Clone)]
pub struct Board {
    pub active_piece: SelectedTetrimino,
}