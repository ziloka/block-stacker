use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
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

pub const TETROMINO_TYPES: [Tetromino; 7] = [
    Tetromino::I,
    Tetromino::J,
    Tetromino::L,
    Tetromino::O,
    Tetromino::S,
    Tetromino::T,
    Tetromino::Z,
];

// https://en.wikipedia.org/wiki/Tetromino
#[derive(Clone, Copy, Debug)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetromino {
    // RGB value of color
    pub fn get_color(&self) -> (u8, u8, u8) {
        match *self {
            Tetromino::I => (0, 255, 255),
            Tetromino::J => (0, 0, 255),
            Tetromino::L => (255, 129, 0),
            Tetromino::O => (255, 255, 0),
            Tetromino::S => (0, 255, 0),
            Tetromino::T => (255, 0, 255),
            Tetromino::Z => (255, 0, 0),
        }
    }

    // https://github.com/zigurous/unity-tetris-tutorial/blob/main/Assets/Scripts/Data.cs
    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L48-L101
    pub fn get_structure(&self) -> [Vec2; 4] {
        match *self {
            Tetromino::I => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(2.0, 0.0),
                vec2(1.0, 0.0),
            ],
            Tetromino::J => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(-1.0, -1.0),
                vec2(1.0, 0.0),
            ],
            Tetromino::L => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(1.0, -1.0),
                vec2(1.0, 0.0),
            ],
            Tetromino::O => [
                // fix
                vec2(0.0, 0.0),
                vec2(1.0, -1.0),
                vec2(0.0, -1.0),
                vec2(1.0, 0.0),
            ],
            Tetromino::S => [
                vec2(0.0, 0.0),
                vec2(0.0, -1.0),
                vec2(-1.0, 0.0),
                vec2(1.0, -1.0),
            ],
            Tetromino::T => [
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(0.0, -1.0),
                vec2(1.0, 0.0),
            ],
            Tetromino::Z => [
                vec2(0.0, 0.0),
                vec2(-1.0, -1.0),
                vec2(0.0, -1.0),
                vec2(1.0, 0.0), 
            ],
        }
    }

    // https://tetris.wiki/Super_Rotation_System
    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L36-L92
    // offset data for the Tetromino wallkicks
    pub fn get_offset_data(&self) -> Vec<[Vec2; 4]> {
        match *self {
            Tetromino::I => [
                [
                    vec2(0.0, 0.0),  // spawn state (O)
                    vec2(-1.0, 0.0), // state (R) resulting from a clockwise rotation ("right") from spawn
                    vec2(-1.0, 1.0), // state (2) resulting from 2 successive rotations in either direction from spawn.
                    vec2(0.0, 1.0), // state (L) resulting from a counter-clockwise ("left") rotation from spawn
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
            Tetromino::O => [[
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
    pub tetromino: Tetromino,
    pub dots: Vec<Vec2>,
    pub rotation_index: i8,
}

pub enum GameState {
    Playing,
    GameOver,
    Paused,
    OpenSettings,
}
