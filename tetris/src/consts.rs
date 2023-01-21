use std::ops::{Add, Sub, Div, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 {
        x,
        y
    }
} 

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub const GRAY: (u8, u8, u8) = (128, 128, 128);
pub const HEIGHT: f32 = 20.0;
pub const WIDTH: f32 = 10.0;
pub const BLOCK_SIZE: f32 = 30.0;

pub const TETRIMINO_TYPES: [Tetrimino; 7] = [
    Tetrimino::I,
    Tetrimino::J,
    Tetrimino::L,
    Tetrimino::O,
    Tetrimino::S,
    Tetrimino::T,
    Tetrimino::Z,
];

// https://en.wikipedia.org/wiki/Tetromino
#[derive(Clone, Copy, Debug)]
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
    // RGB value of color
    pub fn get_color(&self) -> (u8, u8, u8) {
        match *self {
            Tetrimino::I => (0, 181, 226),
            Tetrimino::J => (0, 0, 255),
            Tetrimino::L => (255, 165, 0),
            Tetrimino::O => (255, 255, 0),
            Tetrimino::S => (0, 255, 0),
            Tetrimino::T => (218, 112, 214),
            Tetrimino::Z => (255, 0, 0),
        }
    }

    // https://github.com/zigurous/unity-tetris-tutorial/blob/main/Assets/Scripts/Data.cs
    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L48-L101
    pub fn get_structure(&self) -> [Vec2; 4] {
        match *self {
            Tetrimino::I => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(2.0, 0.0),
                vec2(1.0, 0.0),
            ],
            Tetrimino::J => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(1.0, 0.0),
            ],
            Tetrimino::L => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(-1.0, 1.0),
                vec2(1.0, 0.0),
            ],
            Tetrimino::O => [
                vec2(0.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
                vec2(1.0, 0.0),
            ],
            Tetrimino::S => [
                vec2(0.0, 0.0),
                vec2(0.0, 1.0),
                vec2(-1.0, 1.0),
                vec2(1.0, 0.0),
            ],
            Tetrimino::T => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 0.0),
            ],
            Tetrimino::Z => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
            ],
        }
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L36-L92
    // offset data for the tetrimino wallkicks
    pub fn get_offset_data(&self) -> Vec<[Vec2; 4]> {
        match *self {
            Tetrimino::I => [
                [
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(-1.0, 1.0),
                    vec2(0.0, 1.0),
                ],
                [
                    vec2(-1.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(1.0, 1.0),
                    vec2(0.0, 1.0),
                ],
                [
                    vec2(2.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(-2.0, 1.0),
                    vec2(0.0, 1.0),
                ],
                [
                    vec2(-1.0, 0.0),
                    vec2(0.0, 1.0),
                    vec2(1.0, 0.0),
                    vec2(0.0, -1.0),
                ],
                [
                    vec2(2.0, 0.0),
                    vec2(0.0, -2.0),
                    vec2(-2.0, 0.0),
                    vec2(0.0, 2.0),
                ],
            ]
            .to_vec(),
            Tetrimino::O => [[
                vec2(0.0, 0.0),
                vec2(0.0, -1.0),
                vec2(-1.0, -1.0),
                vec2(-1.0, 0.0),
            ]]
            .to_vec(),
            _ => [
                [
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                ],
                [
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                ],
                [
                    vec2(0.0, 0.0),
                    vec2(1.0, -1.0),
                    vec2(0.0, 0.0),
                    vec2(-1.0, -1.0),
                ],
                [
                    vec2(0.0, 0.0),
                    vec2(0.0, 2.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 2.0),
                ],
                [
                    vec2(0.0, 0.0),
                    vec2(1.0, 2.0),
                    vec2(0.0, 0.0),
                    vec2(-1.0, 2.0),
                ],
            ]
            .to_vec(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub tetrimino: Tetrimino,
    pub dots: Vec<Vec2>,
    pub rotation_index: i8,
}

pub enum GameState {
    Playing,
    GameOver,
    Paused,
    OpenSettings,
}
