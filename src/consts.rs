use macroquad::color::{Color, BLUE, GREEN, ORANGE, PURPLE, RED, SKYBLUE, YELLOW};

pub const HEIGHT: f32 = 20.0;
pub const WIDTH: f32 = 10.0;
pub const BLOCK_SIZE: f32 = 30.0;
// https://www.reddit.com/r/Tetris/comments/frbii6/jstris_das_arr_in_tetrio/
// https://tetris.wiki/DAS
pub const DAS: f32 = 133.0; // Delayed Auto Shift in miliseconds
pub const ARR: f32 = 10.0; // Auto Repeat Rate in miliseconds

pub const TETRIMINO_TYPES: [Tetrimino; 7] = [
    Tetrimino::I,
    Tetrimino::J,
    Tetrimino::L,
    Tetrimino::O,
    Tetrimino::S,
    Tetrimino::T,
    Tetrimino::Z,
];

#[derive(Clone, Copy)]
pub enum Tetrimino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetrimino {
    pub fn get_color(&self) -> Color {
        match *self {
            Tetrimino::I => SKYBLUE,
            Tetrimino::O => YELLOW,
            Tetrimino::T => PURPLE,
            Tetrimino::S => GREEN,
            Tetrimino::Z => RED,
            Tetrimino::J => BLUE,
            Tetrimino::L => ORANGE,
        }
    }

    // https://github.com/zigurous/unity-tetris-tutorial/blob/main/Assets/Scripts/Data.cs
    pub fn get_structure(&self) -> [Position; 4] {
        match *self {
            Tetrimino::I => [
                Position { x: -1.0, y: 1.0 },
                Position { x: 0.0, y: 1.0 },
                Position { x: 1.0, y: 1.0 },
                Position { x: 2.0, y: 1.0 },
            ],
            Tetrimino::O => [
                Position { x: 0.0, y: 1.0 },
                Position { x: 1.0, y: 1.0 },
                Position { x: 0.0, y: 0.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::T => [
                Position { x: 0.0, y: 1.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 0.0, y: 0.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::S => [
                Position { x: 0.0, y: 1.0 },
                Position { x: 1.0, y: 1.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 0.0, y: 0.0 },
            ],
            Tetrimino::Z => [
                Position { x: -1.0, y: 1.0 },
                Position { x: 0.0, y: 1.0 },
                Position { x: 0.0, y: 0.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::J => [
                Position { x: -1.0, y: 1.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 0.0, y: 0.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::L => [
                Position { x: 1.0, y: 1.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 0.0, y: 0.0 },
                Position { x: 1.0, y: 0.0 },
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Piece {
    pub tetrimino: Tetrimino,
    pub position: Position
}