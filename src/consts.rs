use macroquad::{
    color::{Color, BLUE, GREEN, ORANGE, PURPLE, RED, SKYBLUE, YELLOW},
    prelude::{vec2, Vec2},
};

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
    pub fn get_color(&self) -> Color {
        match *self {
            Tetrimino::I => SKYBLUE,
            Tetrimino::J => BLUE,
            Tetrimino::L => ORANGE,
            Tetrimino::O => YELLOW,
            Tetrimino::S => GREEN,
            Tetrimino::T => PURPLE,
            Tetrimino::Z => RED,
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
