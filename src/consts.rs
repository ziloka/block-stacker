use macroquad::{
    color::{Color, BLUE, GREEN, ORANGE, PURPLE, RED, SKYBLUE, YELLOW},
    prelude::{vec2, Vec2},
};

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
    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L48-L101
    pub fn get_structure(&self) -> [Position; 4] {
        match *self {
            Tetrimino::I => [
                Position { x: 0.0, y: 0.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 2.0, y: 0.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            // this isn't really suppose to be here, squares don't change during rotation..
            Tetrimino::O => [
                Position { x: 0.0, y: 0.0 },
                Position { x: 1.0, y: 1.0 },
                Position { x: 0.0, y: 1.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::T => [
                Position { x: 0.0, y: 0.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 0.0, y: 1.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::S => [
                Position { x: 0.0, y: 0.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 1.0, y: 1.0 },
                Position { x: 0.0, y: 1.0 },
            ],
            Tetrimino::Z => [
                Position { x: 0.0, y: 0.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: -1.0, y: 1.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::J => [
                Position { x: 0.0, y: 0.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: -1.0, y: 1.0 },
                Position { x: 1.0, y: 0.0 },
            ],
            Tetrimino::L => [
                Position { x: 0.0, y: 0.0 },
                Position { x: -1.0, y: 0.0 },
                Position { x: 1.0, y: 1.0 },
                Position { x: 1.0, y: 0.0 },
            ],
        }
    }
}

// https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L36-L92
// offset data for the tetrimino wallkicks
pub struct Offset {}

impl Offset {
    pub const JLSTZ: [[Vec2; 4]; 5] = [
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
    ];
    pub const I: [[Vec2; 4]; 5] = [
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
    ];

    pub const O: [[Vec2; 4]; 1] = [[
        vec2(0.0, 0.0),
        vec2(0.0, -1.0),
        vec2(-1.0, -1.0),
        vec2(-1.0, 0.0),
    ]];
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Piece {
    pub tetrimino: Tetrimino,
    pub dots: Vec<Vec2>, // each number is the tile index
    pub rotation_index: i8,
}
