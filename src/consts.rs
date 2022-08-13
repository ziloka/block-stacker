use bevy::prelude::*;

// change handling (measured in frames per movement)
pub const ARR: f32 = 1.6;
pub const DAS: f32 = 8.3;
pub const DCD: f32 = 1.0;
pub const SDF: f32 = std::f32::INFINITY;

struct KEYS;

impl KEYS {
    pub const CLOCKWISE: KeyCode = KeyCode::Up;
    pub const COUNTER_CLOCKWISE: KeyCode = KeyCode::Z;
    pub const MOVE_LEFT: KeyCode = KeyCode::Left;
    pub const MOVE_RIGHT: KeyCode = KeyCode::Right;
    pub const HARDDROP_KEY: KeyCode = KeyCode::Space;
}

pub struct Board {}

impl Board {
    pub const HEIGHT: f32 = 20.0; // 20 squares tall
    pub const WIDTH: f32 = 10.0; // 10 squares wide
    pub const TETRIOMINO_SIDE_LENGTH: f32 = 40.0; // 40 pixels per side (square)
}

pub struct TopRightCorner {}

impl TopRightCorner {
    pub const X: f32 = (Board::WIDTH / 2.0) * Board::TETRIOMINO_SIDE_LENGTH;
    pub const Y: f32 = (Board::HEIGHT / 2.0) * Board::TETRIOMINO_SIDE_LENGTH;
}

// "different data types (Components) are like the "columns" of a table, and there can be arbitrarily many "rows" (Entities) containing values / instances of various components."
#[derive(Component, Clone)]
pub enum TetriminoType {
    I,
    O,
    T,
    S,
    Z,
    J,
}

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
        }
    }
}

pub const TETRIMINO_TYPES: [TetriminoType; 7] = [
    TetriminoType::I,
    TetriminoType::O,
    TetriminoType::T,
    TetriminoType::S,
    TetriminoType::Z,
    TetriminoType::S,
    TetriminoType::J,
];