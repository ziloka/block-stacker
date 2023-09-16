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

impl Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

pub const GRAY: (u8, u8, u8) = (128, 128, 128);
pub const CUSTOM_GARBAGE: (u8, u8, u8) = (105, 105, 105);
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

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino::I
    }
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

    // https://harddrop.com/wiki/File:SRS-true-rotations.png
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
                vec2(0.0, -1.0),
                vec2(-1.0, 0.0),
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

    // https://tetris.wiki/Super_Rotation_System - READ THE "How Guidline SRS *Really* Works" section
    // https://www.youtube.com/watch?v=yIpk5TJ_uaI
    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L36-L92
    // https://github.com/fiorescarlatto/four-tris/blob/dc08ed253e704a4a68302dfc4392b5e28ad3eccf/Tetris.au3#L3395-L3450
    // offset data for the Tetromino wallkicks
    pub fn get_offset_data(&self) -> Vec<Vec<Vec2>> {
        match *self {
            Tetromino::I => vec![
                vec![
                    vec2(0.0, 0.0), // 0: Offset #1
                    vec2(-1.0, 0.0), // R: Offset #1
                    vec2(-1.0, 1.0), // 2: Offset #1
                    vec2(0.0, 1.0), // L: Offset #1
                ],
                vec![
                    vec2(-1.0, 0.0), // 0: Offset #2
                    vec2(0.0, 0.0), // R: Offset #2
                    vec2(1.0, 1.0), // 2: Offset #2
                    vec2(0.0, 1.0), // L: Offset #2
                ],
                vec![
                    vec2(2.0, 0.0), // 0: Offset #3
                    vec2(0.0, 0.0), // R: Offset #3
                    vec2(-2.0, 1.0), // 2: Offset #3
                    vec2(0.0, 1.0), // L: Offset #3
                ],
                vec![
                    vec2(-1.0, 0.0), // 0: Offset #4
                    vec2(0.0, 1.0), // R: Offset #4
                    vec2(1.0, 0.0), // 2: Offset #4
                    vec2(0.0, -1.0), // L: Offset #4
                ],
                vec![
                    vec2(2.0, 0.0), // 0: Offset #5
                    vec2(0.0, -2.0), // R: Offset #5
                    vec2(-2.0, 0.0), // 2: Offset #5
                    vec2(0.0, 2.0), // L: Offset #5
                ],
            ],
            Tetromino::O => vec![vec![
                vec2(0.0, 0.0),
                vec2(0.0, -1.0),
                vec2(-1.0, -1.0),
                vec2(-1.0, 0.0),
            ]],
            _ => vec![
                vec![
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                ],
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                ],
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, -1.0),
                    vec2(0.0, 0.0),
                    vec2(-1.0, -1.0),
                ],
                vec![
                    vec2(0.0, 0.0),
                    vec2(0.0, 2.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 2.0),
                ],
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 2.0),
                    vec2(0.0, 0.0),
                    vec2(-1.0, 2.0),
                ],
            ],
        }
    }

    // 180 wall kicks
    // https://tetris.fandom.com/wiki/SRS#180%C2%B0_rotation
    // nullpomino data: https://github.com/nullpomino/nullpomino/blob/ca361b926cb2f52ce302c7d901b9ca693e830f9f/nullpomino-core/src/main/java/mu/nu/nullpo/game/subsystem/wallkick/StandardWallkick.java#L112-L125
    // nullpomino usage: https://github.com/nullpomino/nullpomino/blob/ca361b926cb2f52ce302c7d901b9ca693e830f9f/nullpomino-core/src/main/java/mu/nu/nullpo/game/subsystem/wallkick/BaseStandardWallkick.java#L33
    // tetrio data: https://cdn.discordapp.com/attachments/674421736162197515/721130601054339072/180kicks.png
    // https://harddrop.com/wiki/List_of_twists#180.C2.B0_Twists
    // https://harddrop.com/wiki/Rotate
    pub fn get_180_offset_data(&self) -> Vec<Vec<Vec2>> {
        match *self {
            Tetromino::I => vec![
                vec![
                    vec2(1.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(1.0, 1.0),
                    vec2(2.0, 1.0),
                    vec2(-1.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(-1.0, 1.0),
                    vec2(-2.0, 1.0),
                    vec2(0.0, -1.0),
                    vec2(3.0, 0.0),
                    vec2(-3.0, 0.0),
                ],
                vec![
                    vec2(0.0, 1.0),
                    vec2(0.0, 2.0),
                    vec2(-1.0, 1.0),
                    vec2(-1.0, 2.0),
                    vec2(0.0, -1.0),
                    vec2(0.0, -2.0),
                    vec2(-1.0, -1.0),
                    vec2(-1.0, -2.0),
                    vec2(1.0, 0.0),
                    vec2(0.0, 3.0),
                    vec2(0.0, -3.0),
                ],
                vec![
                    vec2(1.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(-1.0, -1.0),
                    vec2(-2.0, -1.0),
                    vec2(1.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(1.0, -1.0),
                    vec2(2.0, -1.0),
                    vec2(0.0, 1.0),
                    vec2(-3.0, 0.0),
                    vec2(3.0, 0.0),
                ],
                vec![
                    vec2(0.0, 1.0),
                    vec2(0.0, 2.0),
                    vec2(1.0, 1.0),
                    vec2(1.0, 2.0),
                    vec2(0.0, -1.0),
                    vec2(0.0, -2.0),
                    vec2(1.0, -1.0),
                    vec2(1.0, -2.0),
                    vec2(-1.0, 0.0),
                    vec2(0.0, 3.0),
                    vec2(0.0, -3.0),
                ],
            ],
            _ => vec![
                vec![
                    vec2(-1.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(0.0, 1.0),
                ],
                vec![
                    vec2(0.0, 1.0),
                    vec2(0.0, 2.0),
                    vec2(0.0, -1.0),
                    vec2(0.0, -2.0),
                    vec2(-1.0, 0.0),
                ],
                vec![
                    vec2(1.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(0.0, -1.0),
                ],
                vec![
                    vec2(0.0, 1.0),
                    vec2(0.0, 2.0),
                    vec2(0.0, -1.0),
                    vec2(0.0, -2.0),
                    vec2(1.0, 0.0),
                ],
            ],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Piece {
    pub tetromino: Tetromino,
    pub dots: Vec<Vec2>,
    pub rotation_index: i8,
    pub previous_rotation_index: Option<i8>,
    pub previous_offset_kick: Option<usize>, // there are only 5 kicks
}

pub enum State {
    Playing,
    GameOver,
}
