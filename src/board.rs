use crate::consts::TetriminoType;
use bevy::prelude::Bundle;


#[derive(Bundle, Copy, Clone)]
pub struct Board {
    pub active_piece: TetriminoType,
}