use crate::bricks::{Brick, Dot};
use crate::Color;
use lazy_static::*;

pub(crate) const BACKGROUND: Color = Color::rgb(158.0 / 255.0, 173.0 / 255.0, 135.0 / 255.0);
pub(crate) const WINDOWS_WIDTH: f32 = 360.0;
pub(crate) const WINDOWS_HEIGHT: f32 = 443.0;

pub(crate) const TEXT_SCORE_X: f32 = 248.0;
pub(crate) const TEXT_SCORE_Y: f32 = 48.0;

pub(crate) const TEXT_LINES_X: f32 = 248.0;
pub(crate) const TEXT_LINES_Y: f32 = 126.0;

pub(crate) const TEXT_LEVEL_X: f32 = 248.0;
pub(crate) const TEXT_LEVEL_Y: f32 = 202.0;

pub(crate) const TEXT_GAME_X: f32 = 50.0;
pub(crate) const TEXT_GAME_Y: f32 = 118.0;

pub(crate) const BOARD_X: i8 = 10;
pub(crate) const BOARD_Y: i8 = 23; // board is 10x20
pub(crate) const BOARD_X_Y: usize = 230; // we create 230 for more space for rotate brick.

pub(crate) const BOARD_Y_VALIDE: i8 = 20; // checking for game over

pub(crate) const BOARD_LEFT_PX: f32 = 13.0;
pub(crate) const BOARD_BOTTOM_PX: f32 = 13.0;
pub(crate) const DOT_WIDTH_PX: f32 = 21.0;

pub(crate) const NEXT_BRICK_LEFT_PX: f32 = 263.0;
pub(crate) const NEXT_BRICK_BOTTOM_PX: f32 = 100.0;

pub(crate) const BRICK_START_DOT: Dot = Dot(3, 18);

pub(crate) const BRICKS_TYPES: usize = 7;

pub(crate) const SCORE_PER_DROP: u32 = 10;

// pub(crate) const STRING_GAME_START: &str = "PRESS SPACE";
// pub(crate) const STRING_GAME_PLAYING: &str = "                         ";
pub(crate) const STRING_GAME_OVER: &str = " GAME OVER \n\nPRESS SPACE";

//delay = 725 * .85 ^ level + level (ms)
pub(crate) const TIMER_FALLING_SECS: f32 = 0.725;
pub(crate) const TIMER_KEY_SECS: f32 = 0.100;

lazy_static! {
    pub static ref BRICKS_DICT: Vec<Vec<Brick>> = vec![
        //O:
        vec![Brick{dots:[Dot(1, 1), Dot(1, 2), Dot(2, 1), Dot(2, 2)]}],
        //I:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)]},
            Brick{dots:[Dot(2, 0), Dot(2, 1), Dot(2, 2), Dot(2, 3)]}
        ],
        //J:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(2, 0)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(0, 0)]},
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(0, 2)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(2, 2)]},
        ],
        //L:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(0, 0)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(0, 2)]},
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(2, 2)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(2, 0)]},
        ],
        //S:
        vec![
            Brick{dots:[Dot(0, 0), Dot(1, 0), Dot(1, 1), Dot(2, 1)]},
            Brick{dots:[Dot(1, 2), Dot(1, 1), Dot(2, 1), Dot(2, 0)]},
        ],
        //Z:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(1, 0), Dot(2, 0)]},
            Brick{dots:[Dot(2, 2), Dot(2, 1), Dot(1, 1), Dot(1, 0)]},
        ],
        //T:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(1, 0)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(0, 1)]},
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(1, 2)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(2, 1)]},
        ],
    ];
}

// use crate::piece::Position;
// use bevy::prelude::*;

// // change handling (measured in frames per movement)
// // pub const ARR: f32 = 1.6;
// // pub const DAS: f32 = 8.3;
// // pub const DCD: f32 = 1.0;
// // pub const SDF: f32 = std::f32::INFINITY;

// //delay = 725 * .85 ^ level + level (ms)
// pub(crate) const TIMER_FALLING_SECS: f32 = 0.725;
// pub(crate) const TIMER_KEY_SECS: f32 = 0.100;

// #[derive(PartialEq)]
// pub struct KEYS;

// impl KEYS {
//     pub const CLOCKWISE: KeyCode = KeyCode::Up;
//     pub const COUNTER_CLOCKWISE: KeyCode = KeyCode::Z;
//     pub const MOVE_LEFT: KeyCode = KeyCode::Left;
//     pub const MOVE_RIGHT: KeyCode = KeyCode::Right;
//     pub const SOFTDROP: KeyCode = KeyCode::Down;
//     pub const HARDDROP: KeyCode = KeyCode::Space;
// }

// #[derive(Bundle, Copy, Clone)]
// pub struct BOARD {
//     pub active_piece: TetriminoType,
// }
// impl BOARD {
//     pub const HEIGHT: f32 = 20.0; // 20 blocks high
//     pub const WIDTH: f32 = 10.0; // 10 blocks wide
//     pub const TETRIOMINO_SIDE_LENGTH: f32 = 40.0; // 40 pixels per side (square)
//     pub const TOP_RIGHT_CORNER: Position = Position {
//         // top right corner of the board
//         x: BOARD::WIDTH * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0,
//         y: BOARD::HEIGHT * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0,
//     };
//     pub const BOTTOM_LEFT_CORNER: Position = Position {
//         x: BOARD::WIDTH * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0 * -1.0,
//         y: BOARD::HEIGHT * BOARD::TETRIOMINO_SIDE_LENGTH / 2.0 * -1.0,
//     };
// }
// #[derive(Copy, Clone)]
// pub struct Delta {
//     pub x: f32,
//     pub y: f32,
// }

// #[derive(Component, Copy, Clone)]
// pub struct ActivePiece;

// // https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
// // https://stackoverflow.com/a/31013047
// // By the way, every Copy type is also required to be Clone. However, they are not required to do the same thing! For your own types, .clone() can be an arbitrary method of your choice, whereas implicit copying will always trigger a memcpy, not the clone(&self) implementation.
// #[derive(Component, Copy, Clone, Debug)]
// pub enum TetriminoType {
//     I,
//     O,
//     T,
//     S,
//     Z,
//     J,
//     L,
// }

// pub const TETRIMINO_TYPES: [TetriminoType; 7] = [
//     TetriminoType::I,
//     TetriminoType::O,
//     TetriminoType::T,
//     TetriminoType::Z,
//     TetriminoType::S,
//     TetriminoType::J,
//     TetriminoType::L,
// ];

// // https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
// // https://stackoverflow.com/a/36928678
// impl TetriminoType {
//     pub fn get_color(&self) -> Color {
//         match *self {
//             TetriminoType::I => Color::CYAN,
//             TetriminoType::O => Color::YELLOW,
//             TetriminoType::T => Color::PURPLE,
//             TetriminoType::S => Color::GREEN,
//             TetriminoType::Z => Color::RED,
//             TetriminoType::J => Color::BLUE,
//             TetriminoType::L => Color::ORANGE,
//         }
//     }

//     // https://github.com/zigurous/unity-tetris-tutorial/blob/main/Assets/Scripts/Data.cs
//     pub fn get_structure(&self) -> [Delta; 4] {
//         match *self {
//             TetriminoType::I => [
//                 Delta { x: -1.0, y: 1.0 },
//                 Delta { x: 0.0, y: 1.0 },
//                 Delta { x: 1.0, y: 1.0 },
//                 Delta { x: 2.0, y: 1.0 },
//             ],
//             TetriminoType::O => [
//                 Delta { x: 0.0, y: 1.0 },
//                 Delta { x: 1.0, y: 1.0 },
//                 Delta { x: 0.0, y: 0.0 },
//                 Delta { x: 1.0, y: 0.0 },
//             ],
//             TetriminoType::T => [
//                 Delta { x: 0.0, y: 1.0 },
//                 Delta { x: -1.0, y: 0.0 },
//                 Delta { x: 0.0, y: 0.0 },
//                 Delta { x: 1.0, y: 0.0 },
//             ],
//             TetriminoType::S => [
//                 Delta { x: 0.0, y: 1.0 },
//                 Delta { x: 1.0, y: 1.0 },
//                 Delta { x: -1.0, y: 0.0 },
//                 Delta { x: 0.0, y: 0.0 },
//             ],
//             TetriminoType::Z => [
//                 Delta { x: -1.0, y: 1.0 },
//                 Delta { x: 0.0, y: 1.0 },
//                 Delta { x: 0.0, y: 0.0 },
//                 Delta { x: 1.0, y: 0.0 },
//             ],
//             TetriminoType::J => [
//                 Delta { x: -1.0, y: 1.0 },
//                 Delta { x: -1.0, y: 0.0 },
//                 Delta { x: 0.0, y: 0.0 },
//                 Delta { x: 1.0, y: 0.0 },
//             ],
//             TetriminoType::L => [
//                 Delta { x: 1.0, y: 1.0 },
//                 Delta { x: -1.0, y: 0.0 },
//                 Delta { x: 0.0, y: 0.0 },
//                 Delta { x: 1.0, y: 0.0 },
//             ],
//         }
//     }
// }
