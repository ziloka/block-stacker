use bevy::prelude::*;

pub const BOARD_HEIGHT: f32 = 20.0; // 20 squares tall
pub const BOARD_WIDTH: f32 = 10.0; // 10 squares wide
pub const TETRIOMINO_SIDE_LENGTH: f32 = 40.0;

pub struct bottom_left_corner {
    pub X: f32,
    pub Y: f32,
}

pub const BOTTOM_LEFT_CORNER: bottom_left_corner = bottom_left_corner {
    X: (BOARD_WIDTH / 2.0) * TETRIOMINO_SIDE_LENGTH,
    Y: (BOARD_HEIGHT / 2.0) * TETRIOMINO_SIDE_LENGTH
};

// "different data types (Components) are like the "columns" of a table, and there can be arbitrarily many "rows" (Entities) containing values / instances of various components."
#[derive(Component)]
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
