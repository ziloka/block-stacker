use macroquad::miniquad::date::now;
use tetris::{board::Board, consts::Vec2};
use crate::{drawer::Drawer, input::Input};

pub struct Game {
    pub board: Board,
    pub drawer: Drawer,
    pub input: Input,
}

impl Game {
    pub fn new(left_top_corner: Vec2) -> Self {
        Self {
            board: Board::new(now() as usize),
            drawer: Drawer {
                left_top_corner
            },
            input: Input::default(),
        }
    }
}