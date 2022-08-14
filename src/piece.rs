use crate::consts::TetriminoType;

#[derive(Copy, Clone)]
pub struct Position {
    pub X: f32,
    pub Y: f32,
}

struct piece {
    position: Position,
    locked: bool,
    hold: bool,
    tetrimino_type: TetriminoType
}