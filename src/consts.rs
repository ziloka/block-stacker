use bevy::prelude::*;
use crate::piece::Position;

// change handling (measured in frames per movement)
// pub const ARR: f32 = 1.6;
// pub const DAS: f32 = 8.3;
// pub const DCD: f32 = 1.0;
// pub const SDF: f32 = std::f32::INFINITY;

#[derive(PartialEq)]
pub struct KEYS;

impl KEYS {
    pub const CLOCKWISE: KeyCode = KeyCode::Up;
    pub const COUNTER_CLOCKWISE: KeyCode = KeyCode::Z;
    pub const MOVE_LEFT: KeyCode = KeyCode::Left;
    pub const MOVE_RIGHT: KeyCode = KeyCode::Right;
    pub const SOFTDROP: KeyCode = KeyCode::Down;
    pub const HARDDROP: KeyCode = KeyCode::Space;
}


#[derive(Bundle, Copy, Clone)]
pub struct BOARD {
  pub active_piece: TetriminoType,
}
impl BOARD {
    pub const HEIGHT: f32 = 20.0; // 20 blocks high
    pub const WIDTH: f32 = 10.0; // 10 blocks wide
    pub const TETRIOMINO_SIDE_LENGTH: f32 = 40.0; // 40 pixels per side (square)
    pub const TOP_RIGHT_CORNER: Position = Position { // top right corner of the board
      x: BOARD::WIDTH * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0,
      y: BOARD::HEIGHT * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0,
    };
    pub const BOTTOM_LEFT_CORNER: Position = Position {
      x: BOARD::WIDTH * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0 * -1.0,
      y: BOARD::HEIGHT * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0 * -1.0,
    };
}
#[derive(Copy, Clone)]
pub struct Delta {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Copy, Clone)]
pub struct ActivePiece;

// https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
// https://stackoverflow.com/a/31013047
// By the way, every Copy type is also required to be Clone. However, they are not required to do the same thing! For your own types, .clone() can be an arbitrary method of your choice, whereas implicit copying will always trigger a memcpy, not the clone(&self) implementation.
#[derive(Component, Copy, Clone, Debug)]
pub enum TetriminoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub const TETRIMINO_TYPES: [TetriminoType; 8] = [
    TetriminoType::I,
    TetriminoType::O,
    TetriminoType::T,
    TetriminoType::S,
    TetriminoType::Z,
    TetriminoType::S,
    TetriminoType::J,
    TetriminoType::L,
];

// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
// https://stackoverflow.com/a/36928678
impl TetriminoType {
    pub fn get_color(&self) -> Color {
        match *self {
            TetriminoType::I => Color::CYAN,
            TetriminoType::O => Color::YELLOW,
            TetriminoType::T => Color::PURPLE,
            TetriminoType::S => Color::GREEN,
            TetriminoType::Z => Color::RED,
            TetriminoType::J => Color::BLUE,
            TetriminoType::L => Color::ORANGE,
        }
    }

    // https://github.com/zigurous/unity-tetris-tutorial/blob/main/Assets/Scripts/Data.cs
    pub fn get_structure(&self) -> [Delta; 4] {
        match *self {
            TetriminoType::I => [
                Delta { x: -1.0, y: 1.0 },
                Delta { x: 0.0, y: 1.0 },
                Delta { x: 1.0, y: 1.0 },
                Delta { x: 2.0, y: 1.0 },
            ],
            TetriminoType::O => [
              Delta { x: 0.0, y: 1.0 },
              Delta { x: 1.0, y: 1.0 },
              Delta { x: 0.0, y: 0.0 },
              Delta { x: 1.0, y: 0.0 },
            ],
            TetriminoType::T => [
              Delta { x: 0.0, y: 1.0 },
              Delta { x: -1.0, y: 0.0 },
              Delta { x: 0.0, y: 0.0 },
              Delta { x: 1.0, y: 0.0 },
            ],
            TetriminoType::S => [
                Delta { x: 0.0, y: 1.0 },
                Delta { x: 1.0, y: 1.0 },
                Delta { x: -1.0, y: 0.0 },
                Delta { x: 0.0, y: 0.0 },
            ],
            TetriminoType::Z => [
                Delta { x: -1.0, y: 1.0 },
                Delta { x: 0.0, y: 1.0 },
                Delta { x: 0.0, y: 0.0 },
                Delta { x: 1.0, y: 0.0 },
            ],
            TetriminoType::J => [
                Delta { x: -1.0, y: 1.0 },
                Delta { x: -1.0, y: 0.0 },
                Delta { x: 0.0, y: 0.0 },
                Delta { x: 1.0, y: 0.0 },
            ],
            TetriminoType::L => [
                Delta { x: 1.0, y: 1.0 },
                Delta { x: -1.0, y: 0.0 },
                Delta { x: 0.0, y: 0.0 },
                Delta { x: 1.0, y: 0.0 },
            ],
        }
    }
}
