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
#[derive(Default, Clone, Copy, Debug)]
pub enum Tetromino {
    #[default]
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

// impl Default for Tetromino {
//     fn default() -> Self {
//         Tetromino::I
//     }
// }

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

    // figure out what index to recieve from the offset table
    // for 90 degree turns
    pub fn find_offset_row_90(first: i8, second: i8) -> usize {
        if (first, second) == (0, 1) {
            // 0->R
            0
        } else if (first, second) == (1, 0) {
            // R->0
            1
        } else if (first, second) == (1, 2) {
            // R->2
            2
        } else if (first, second) == (2, 1) {
            // 2->R
            3
        } else if (first, second) == (2, 3) {
            // 2->L
            4
        } else if (first, second) == (3, 2) {
            // L->2
            5
        } else if (first, second) == (3, 0) {
            // L->0
            6
        } else if (first, second) == (0, 3) {
            // 0->L
            7
        } else {
            println!("first: {}, second: {}", first, second);
            unimplemented!();
        }
    }

    // https://tetris.wiki/Super_Rotation_System
    // determine the rotation index of the tetromino
    // first number is the number representation, the second is how tetris.wiki refers to the tetromino states
    // returning 0 - 0 means it is in its spawn position
    // returning 1 - R means it is in a clockwise rotation ("right") from spawn
    // returning 3 - L means it is in a counter-clockwise rotation ("left") from spawn
    // returning 2 - 2 means it is in a 180 degree rotation from spawn
    // offset data for the Tetromino wallkicks
    pub fn get_offset_data(&self) -> Vec<Vec<Vec2>> {
        match *self {
            Tetromino::I => vec![
                vec![
                    vec2(0.0, 0.0),   // test #1
                    vec2(-2.0, 0.0),  // test #2
                    vec2(1.0, 0.0),   // test #3
                    vec2(-2.0, -1.0), // test #4
                    vec2(1.0, 2.0),   // test #5
                ], // 0->R
                vec![
                    vec2(0.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(2.0, 1.0),
                    vec2(-1.0, -2.0),
                ], // R->0
                vec![
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(-1.0, 2.0),
                    vec2(2.0, -1.0),
                ], // R->2
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(1.0, -2.0),
                    vec2(-2.0, 1.0),
                ], // 2->R
                vec![
                    vec2(0.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(2.0, 1.0),
                    vec2(-1.0, -2.0),
                ], // 2->L
                vec![
                    vec2(0.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(-2.0, -1.0),
                    vec2(1.0, 2.0),
                ], // L->2
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(-2.0, 0.0),
                    vec2(1.0, -2.0),
                    vec2(-2.0, 1.0),
                ], // L->0
                vec![
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(-1.0, 2.0),
                    vec2(2.0, -1.0),
                ], // 0->L
            ],
            Tetromino::O => vec![
                vec![
                    // 0->R (0->1)
                    vec2(0.0, 1.0),
                ],
                vec![
                    // R->0
                    vec2(0.0, -1.0),
                ],
                vec![
                    // R->2 (1->2)
                    vec2(1.0, 0.0),
                ],
                vec![
                    // 2->R
                    vec2(-1.0, 0.0),
                ],
                vec![
                    // 2->L (2->3)
                    vec2(0.0, -1.0),
                ],
                vec![
                    // L->2
                    vec2(0.0, 1.0),
                ],
                vec![
                    // L->0 (3->0)
                    vec2(-1.0, 0.0),
                ],
                vec![
                    // 0->L
                    vec2(1.0, 0.0),
                ],
            ],
            _ => vec![
                vec![
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(-1.0, 1.0),
                    vec2(0.0, -2.0),
                    vec2(-1.0, -2.0),
                ], // 0->R
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(1.0, -1.0),
                    vec2(0.0, 2.0),
                    vec2(1.0, 2.0),
                ], // R->0
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(1.0, -1.0),
                    vec2(0.0, 2.0),
                    vec2(1.0, 2.0),
                ], // R->2
                vec![
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(-1.0, 1.0),
                    vec2(0.0, -2.0),
                    vec2(-1.0, -2.0),
                ], // 2->R
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(1.0, 1.0),
                    vec2(0.0, -2.0),
                    vec2(1.0, -2.0),
                ], // 2->L
                vec![
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(-1.0, -1.0),
                    vec2(0.0, 2.0),
                    vec2(-1.0, 2.0),
                ], // L->2
                vec![
                    vec2(0.0, 0.0),
                    vec2(-1.0, 0.0),
                    vec2(-1.0, -1.0),
                    vec2(0.0, 2.0),
                    vec2(-1.0, 2.0),
                ], // L->0
                vec![
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(1.0, 1.0),
                    vec2(0.0, -2.0),
                    vec2(1.0, -2.0),
                ], // 0->L
            ],
        }
    }

    pub fn find_offset_row_180(first: i8, second: i8) -> usize {
        if (first, second) == (0, 2) {
            // 0->L (0>>2)
            0
        } else if (first, second) == (1, 3) {
            // L->0 (1>>3)
            1
        } else if (first, second) == (2, 0) {
            // R->3 (2>>0)
            2
        } else if (first, second) == (3, 1) {
            // 3->R (3>>1)
            3
        } else {
            println!("first: {}, second: {}", first, second);
            unimplemented!();
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

#[derive(Clone)]
pub enum State {
    Playing,
    GameOver,
}
